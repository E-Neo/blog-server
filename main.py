#!/usr/bin/env python

import bcrypt
import psycopg2
import requests
import urllib.parse
import os.path
import dateutil.parser
import tornado.escape
import tornado.httpclient
import tornado.httpserver
import tornado.ioloop
import tornado.options
import tornado.web
import tornado.netutil
from tornado import gen
from bs4 import BeautifulSoup
from tornado.escape import json_encode
from tornado.options import define, options

define("port", default=8000, help="run on the given port", type=int)


class Application(tornado.web.Application):

    def __init__(self):
        handlers = [
            (r"/", MainHandler),
            (r"/admin/", AdminHandler),
            (r"/admin/api/([^/]+)", AdminAPIHandler),
            (r"/admin/blogs", AdminBlogIndexHandler),
            (r"/admin/blog/([^/]+)", AdminBlogHandler),
            (r"/admin/tweets", AdminTweetHandler),
            (r"/api/tweets", APITweetHandler),
            (r"/api/blogs", APIBlogIndexHandler),
            (r"/api/blog/([^/]+)", APIBlogHandler),
            (r"/api/ip", APIIPHandler),
            (r"/api/ip/([^/]+)", APIIPInfoHandler),
            (r"/api/proxy", APIProxyHandler),
            (r"/blogs", BlogIndexHandler),
            (r"/blog/([^/]+)", BlogHandler),
            (r"/login", LoginHandler),
            (r"/logout", LogoutHandler),
            (r"/tweets", TweetIndexHandler)
        ]
        settings = dict(
            template_path=os.path.join(os.path.dirname(__file__), "templates"),
            static_path=os.path.join(os.path.dirname(__file__), "static"),
            cookie_secret="Generate your own cookie secret here.",
            login_url="/login"
        )
        super(Application, self).__init__(handlers, **settings)
        url = urllib.parse.urlparse(os.environ["DATABASE_URL"])
        self.db = psycopg2.connect(
            database=url.path[1:],
            user=url.username,
            password=url.password,
            host=url.hostname,
            port=url.port
        )
        cursor = self.db.cursor()
        cursor.execute("set time zone 'Asia/Shanghai';")
        self.danger_level = 0


class BaseHandler(tornado.web.RequestHandler):

    @property
    def db(self):
        return self.application.db

    def get_current_user(self):
        cursor = self.db.cursor()
        user_id = self.get_secure_cookie("E-Neo")
        if not user_id:
            return None
        cursor.execute("select * from authors where id = (%s)",
                       (int(user_id), ))
        author = cursor.fetchone()
        return author


class APITweetHandler(BaseHandler):

    def get(self):
        cursor = self.db.cursor()
        cursor.execute("select * from tweets;")
        tweets = cursor.fetchall()
        data = dict()
        if tweets:
            for i in tweets:
                data[i[0]] = [i[1], i[2], str(i[3])]
        self.set_header('Content-Type', 'application/javascript')
        self.write(json_encode(data))

    def post(self):
        current = self.get_argument("current")
        try:
            current = dateutil.parser.parse(current)
            cursor = self.db.cursor()
            cursor.execute("select * from tweets where published < "
                           "(%s) order by published desc limit 10;",
                           (current, ))
            tweets = cursor.fetchall()
            data = dict()
            if tweets:
                for i in tweets:
                    data[i[0]] = [i[1], i[2], str(i[3])]
            self.set_header('Content-Type', 'application/javascript')
            self.write(json_encode(data))
        except:
            raise tornado.web.HTTPError(400)


class APIBlogIndexHandler(BaseHandler):

    def get(self):
        cursor = self.db.cursor()
        cursor.execute("select slug, title, published from blogs;")
        blogs = cursor.fetchall()
        data = dict()
        if blogs:
            for i in blogs:
                data[i[0]] = [i[1], str(i[2])]
        self.set_header('Content-Type', 'application/javascript')
        self.write(json_encode(data))


class APIBlogHandler(BaseHandler):

    def get(self, slug):
        cursor = self.db.cursor()
        cursor.execute("select * from blogs where slug = (%s);",
                       (slug, ))
        blog = cursor.fetchone()
        if not blog:
            raise tornado.web.HTTPError(404)
        else:
            data = dict()
            data[blog[0]] = [blog[1], blog[2], blog[3], str(blog[4])]
            self.set_header('Content-Type', 'application/javascript')
            self.write(json_encode(data))


class APIIPHandler(BaseHandler):

    def get(self):
        ip = self.request.headers.get("X-Forwarded-For",
                                      self.request.remote_ip)
        ip = ip.split(',')[-1].strip()
        ip = self.request.headers.get("X-Real-Ip", ip)
        if tornado.netutil.is_valid_ip(ip):
            self.request.remote_ip = ip
        self.write(self.request.remote_ip)


class APIIPInfoHandler(BaseHandler):

    def get(self, ip):
        if tornado.netutil.is_valid_ip(ip):
            r = requests.get('https://www.ip2location.com/demo/'
                             + ip)
            try:
                data = dict()
                soup = BeautifulSoup(r.text, 'html.parser')
                rows = soup.table.tbody.find_all('tr')[:16]
                for row in rows:
                    td = row.find_all('td')
                    data[td[0].b.text.strip()] = td[1].text.strip()
                self.set_header('Content-Type', 'application/javascript')
                self.write(json_encode(data))
            except:
                raise tornado.web.HTTPError(404)
        else:
            raise tornado.web.HTTPError(404)


class APIProxyHandler(BaseHandler):

    @gen.coroutine
    def get(self):
        url = self.get_argument("url")
        cursor = self.db.cursor()
        cursor.execute("select url from proxy_white "
                       "where url = (%s)", (url, ))
        if cursor.fetchone():
            try:
                client = tornado.httpclient.AsyncHTTPClient()
                r = yield client.fetch(url)
                for k, v in r.headers.get_all():
                    self.set_header(k, v)
                self.write(r.body)
            except:
                raise tornado.web.HTTPError(404)
        else:
            raise tornado.web.HTTPError(404)


class MainHandler(BaseHandler):

    def get(self):
        cursor = self.db.cursor()
        cursor.execute("select html from pages "
                       "where slug = 'index';")
        text = cursor.fetchone()[0]
        self.render("index.html", text=text)


class LoginHandler(BaseHandler):

    def get(self):
        if self.application.danger_level <= 2:
            self.render("login.html")
        else:
            raise tornado.web.HTTPError(404)

    def post(self):
        if self.application.danger_level <= 2:
            cursor = self.db.cursor()
            email = self.get_argument("email")
            password = tornado.escape.utf8(self.get_argument("password"))
            cursor.execute("select * from authors "
                           "where email = (%s)", (email, ))
            author = cursor.fetchone()
            wikipedia_url = "https://en.wikipedia.org/wiki/Special:Random"
            if not author:
                self.application.danger_level += 1
                self.redirect(wikipedia_url)
                return
            hashed_password = tornado.escape.utf8(author[3])
            if bcrypt.checkpw(password, hashed_password):
                self.application.danger_level = 0
                self.set_secure_cookie("E-Neo", str(author[0]))
                self.redirect("/admin/")
            else:
                self.application.danger_level += 1
                self.redirect(wikipedia_url)
        else:
            raise tornado.web.HTTPError(405)


class LogoutHandler(BaseHandler):

    def get(self):
        if not self.current_user:
            raise tornado.web.HTTPError(404)
        else:
            self.clear_cookie("E-Neo")
            self.redirect("/")


class BlogIndexHandler(BaseHandler):

    def get(self):
        self.render("blog.html")


class BlogHandler(BaseHandler):

    def get(self, slug):
        self.render("blog_entry.html", slug=slug)


class TweetIndexHandler(BaseHandler):

    def get(self):
        self.render("tweet.html")


class AdminHandler(BaseHandler):

    def get(self):
        if not self.current_user:
            raise tornado.web.HTTPError(404)
        else:
            self.render("admin/admin.html")

    def post(self):
        if not self.current_user:
            raise tornado.web.HTTPError(405)
        else:
            type = self.get_argument("type")
            if type == "edit-index":
                cursor = self.db.cursor()
                html = self.get_argument("html")
                cursor.execute("update pages set html = (%s) "
                               "where slug = 'index'",
                               (html, ))
                self.db.commit()
            else:
                raise tornado.web.HTTPError(400)


class AdminAPIHandler(BaseHandler):

    def get(self, service):
        if not self.current_user:
            raise tornado.web.HTTPError(404)
        else:
            if service == 'index':
                cursor = self.db.cursor()
                cursor.execute("select html from pages "
                               "where slug = 'index';")
                html = cursor.fetchone()[0]
                data = dict()
                data["index"] = html
                self.set_header('Content-Type', 'application/javascript')
                self.write(json_encode(data))
            else:
                raise tornado.web.HTTPError(404)


class AdminBlogIndexHandler(BaseHandler):

    def get(self):
        if not self.current_user:
            raise tornado.web.HTTPError(404)
        else:
            self.render("admin/blog.html")

    def post(self):
        if not self.current_user:
            raise tornado.web.HTTPError(405)
        else:
            type = self.get_argument("type")
            if type == "new":
                cursor = self.db.cursor()
                title = self.get_argument("title")
                text = self.get_argument("markdown")
                if len(title) == 0 or len(text) == 0:
                    raise tornado.web.HTTPError(400)
                cursor.execute("insert into blogs (slug, author_id, title, "
                               "markdown, published) values(to_char("
                               "current_timestamp, 'YYYYMMDDHH24MISSTZ'), 1, "
                               "(%s), (%s), (select current_timestamp));",
                               (title, text))
                self.db.commit()
                self.redirect("/admin/blogs")
            elif type == "edit":
                cursor = self.db.cursor()
                slug = self.get_argument("slug")
                title = self.get_argument("title")
                text = self.get_argument("markdown")
                cursor.execute("update blogs set (title, markdown)"
                               " = ((%s), (%s)) where slug = (%s);",
                               (title, text, slug))
                self.db.commit()
            elif type == "delete":
                cursor = self.db.cursor()
                slug = self.get_argument("slug")
                cursor.execute("delete from blogs where slug = (%s);",
                               (slug, ))
                self.db.commit()
            else:
                raise tornado.web.HTTPError(400)


class AdminBlogHandler(BaseHandler):

    def get(self, slug):
        if not self.current_user:
            raise tornado.web.HTTPError(404)
        else:
            self.render("admin/blog_entry.html", slug=slug)


class AdminTweetHandler(BaseHandler):

    def get(self):
        if not self.current_user:
            raise tornado.web.HTTPError(404)
        else:
            self.render("admin/tweet.html")

    def post(self):
        if not self.current_user:
            raise tornado.web.HTTPError(405)
        else:
            type = self.get_argument("type")
            if type == "new":
                cursor = self.db.cursor()
                text = self.get_argument("markdown")
                cursor.execute("insert into tweets (slug, author_id, "
                               "markdown, published) values(to_char("
                               "current_timestamp, 'YYYYMMDDHH24MISSTZ'), 1, "
                               "(%s), (select current_timestamp));",
                               (text, ))
                self.db.commit()
                self.redirect("/admin/tweets")
            elif type == "delete":
                cursor = self.db.cursor()
                slug = self.get_argument("slug")
                cursor.execute("delete from tweets where slug = (%s);",
                               (slug, ))
                self.db.commit()
            elif type == "edit":
                cursor = self.db.cursor()
                slug = self.get_argument("slug")
                text = self.get_argument("markdown")
                cursor.execute("update tweets set markdown = (%s) "
                               "where slug = (%s);",
                               (text, slug))
                self.db.commit()
            else:
                raise tornado.web.HTTPError(400)


def main():
    tornado.options.parse_command_line()
    app = Application()
    app.listen(options.port)
    tornado.ioloop.IOLoop.current().start()


if __name__ == "__main__":
    main()
