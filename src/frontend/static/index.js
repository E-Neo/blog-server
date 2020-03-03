(function() {
    const MAX_LENGTH = 20;

    function request(method, url, body, callback, contentType) {
        let req = new XMLHttpRequest();
        req.onreadystatechange = function() {
            if (req.readyState == 4)
                callback(req);
        }
        req.open(method, url, true);
        if (contentType)
            req.setRequestHeader('Content-Type', contentType);
        req.send(body);
    }

    function getBeforeTime() {
        return ((new Date()).getUTCFullYear() + 1) + '-01-01T00:00:00';
    }

    function formatDate(d) {
        let month = d.getMonth() + 1;
        let date = d.getDate();
        let hours = d.getHours();
        let minutes = d.getMinutes();
        let seconds = d.getSeconds();
        return d.getFullYear() + '-' +
            (month >= 10 ? month : '0' + month) + '-' +
            (date >= 10 ? date : '0' + date) + ' ' +
            (hours >= 10 ? hours : '0' + hours) + ':' +
            (minutes >= 10 ? minutes : '0' + minutes) + ':' +
            (seconds >= 10 ? seconds : '0' + seconds);
    }

    function setNavItem(id, url, active) {
        let nav = document.getElementById(id);
        nav.href = url;
        if (active) {
            nav.classList.add('active');
        }
    }

    function clearMain() {
        let main = document.getElementById('main');
        main.innerHTML = '';
    }

    function addHeading(content) {
        let main = document.getElementById('main');
        let h1 = document.createElement('h1');
        h1.innerHTML = content;
        h1.style.textAlign = 'center';
        main.appendChild(h1);
    }

    function addLoading() {
        let main = document.getElementById('main');
        let h1 = document.createElement('h1');
        h1.innerHTML = 'Loading...';
        h1.id = 'loading';
        h1.style.textAlign = 'center';
        main.appendChild(h1);
    }

    function removeLoading() {
        let loading = document.getElementById('loading');
        loading.parentNode.removeChild(loading);
    }

    function addBottomLine() {
        let p = document.createElement('p');
        p.id = 'bottom-line';
        p.innerHTML = '--- I am the bottom line ---';
        p.style.textAlign = 'center';
        document.getElementById('main').appendChild(p);
    }

    function addLoadMore(callback) {
        let button = document.createElement('button');
        button.onclick = function() {
            button.parentNode.removeChild(button);
            addLoading();
            callback();
        }
        button.id = 'load-more';
        button.innerHTML = 'More';
        button.classList.add('btn');
        document.getElementById('main').appendChild(button);
    }

    function createUl() {
        let ul = document.createElement('ul');
        ul.classList.add('list-group');
        return ul;
    }

    function createLi() {
        let li = document.createElement('li');
        li.classList.add('list-group-item');
        return li;
    }

    function createButton(content, onclick) {
        let button = document.createElement('button');
        button.classList.add('btn');
        button.innerHTML = content;
        button.onclick = onclick;
        return button;
    }

    function createTextarea(content) {
        let textarea = document.createElement('textarea');
        textarea.value = content;
        textarea.style.display = 'block';
        textarea.style.width = '100%';
        textarea.style.height = '128px';
        return textarea;
    }

    function getTweets(beforeTime, callback) {
        request(
            'GET',
            '/api/tweet/?before_time=' + beforeTime,
            '',
            function(req) {
                if (req.status == 200) {
                    callback(JSON.parse(req.responseText));
                }
            });
    }

    function getBlogs(beforeTime, callback) {
        request(
            'GET',
            '/api/blog/?before_time=' + beforeTime,
            '',
            function(req) {
                if (req.status == 200) {
                    callback(JSON.parse(req.responseText));
                }
            });
    }

    function home() {
        setNavItem('nav-home', '#', true);
        request('GET', '/api/page/home', '', function(req) {
            if (req.status == 200) {
                document.getElementById('main').innerHTML =
                    marked(JSON.parse(req.responseText).markdown);
            }
        });
    }

    function tweet() {
        function render(tweets) {
            removeLoading();
            let main = document.getElementById('main');
            let ul = document.getElementById('tweets');
            if (ul == null) {
                ul = document.createElement('ul');
                ul.id = 'tweets';
                ul.classList.add('list-group');
                main.appendChild(ul);
            }
            tweets.forEach(function(tweet) {
                let li = document.createElement('li');
                let div1 = document.createElement('div');
                div1.innerHTML = marked(tweet.markdown);
                let div2 = document.createElement('div');
                let p = document.createElement('p');
                p.innerHTML = formatDate(new Date(tweet.created_time + 'Z'));
                p.style.fontSize = 'small';
                p.style.textAlign = 'right';
                p.style.textDecoration = 'overline';
                div2.appendChild(p);
                li.appendChild(div1);
                li.appendChild(div2);
                li.classList.add('list-group-item');
                ul.appendChild(li);
            });
            if (tweets.length == MAX_LENGTH) {
                addLoadMore(function() {
                    getTweets(tweets[tweets.length - 1].created_time, render);
                });
            } else {
                addBottomLine();
            }
        }

        setNavItem('nav-tweet', '#', true);
        getTweets(getBeforeTime(), render);
    }

    function blog() {
        function renderBlogs(blogs) {
            function createBlogItem(blog) {
                let li = createLi();
                let a = document.createElement('a');
                li.appendChild(a);
                a.href = '/blog/' + blog.id;
                a.innerHTML = marked(blog.title);
                return li;
            }

            removeLoading();
            let ul = document.querySelector('#blogs');
            blogs.forEach(function(blog) {
                ul.appendChild(createBlogItem(blog));
            });
            if (blogs.length == MAX_LENGTH) {
                addLoadMore(function() {
                    getBlogs(blogs[blogs.length - 1].created_time, renderBlogs)
                });
            } else {
                addBottomLine();
            }
        }

        setNavItem('nav-blog', '#', true);
        let ul = createUl();
        ul.id = 'blogs';
        document.querySelector('#main').appendChild(ul);
        getBlogs(getBeforeTime(), function(blogs) {
            renderBlogs(blogs);
        });
    }

    function blogDetail(id) {
        setNavItem('nav-blog', '/blog/', true);
        request('GET', '/api/blog/' + id, '', function(req) {
            removeLoading();
            let main = document.getElementById('main');
            if (req.status == 200) {
                let blog = JSON.parse(req.responseText);
                let div1 = document.createElement('div');
                div1.innerHTML = marked('# ' + blog.title);
                main.appendChild(div1);
                let div2 = document.createElement('div');
                div2.innerHTML = marked(blog.markdown);
                main.appendChild(div2);
            } else if (req.status == 400) {
                addHeading('Invalid Blog ID!');
            } else {
                addHeading('Ridiculous!');
            }
        });
    }

    function login() {
        removeLoading();
        let main = document.getElementById('main');
        main.innerHTML = '<ul class="list-group">' +
            '<li class="list-group-item">' +
            '<label for="username">Username: </label>' +
            '<input id="username" type="text" name="username"></li>' +
            '<li class="list-group-item">' +
            '<label for="password">Password: </label>' +
            '<input id="password" type="password" name="password"></li>' +
            '<li class="list-group-item">' +
            '<button id="login" type="button">Login</button></li>' +
            '</ul>';
        document.getElementById('login').onclick = function() {
            let username = document.getElementById('username').value;
            let password = document.getElementById('password').value;
            clearMain();
            addLoading();
            request(
                'POST',
                '/api/auth',
                JSON.stringify({'username': username, 'password': password}),
                function(req) {
                    removeLoading();
                    if (req.status == 200) {
                        window.location.replace('/admin/');
                    } else if (req.status == 401) {
                        addHeading('Unauthorized!');
                    } else {
                        addHeading('Ridiculous!');
                    }
                },
                'application/json'
            );
        }
    }

    function logout() {
        request('DELETE', '/api/auth', '', function(req) {
            removeLoading();
            let main = document.getElementById('main');
            if (req.status == 200) {
                window.location.replace('/login');
            } else if (req.status == 401) {
                addHeading('Unauthorized!');
            } else {
                addHeading('Ridiculous!');
            }
        });
    }

    function admin() {
        function render(markdown) {
            let main = document.getElementById('main');
            let ul = createUl();
            main.appendChild(ul);
            let li = createLi();
            ul.appendChild(li);
            let label = document.createElement('label');
            li.appendChild(label);
            label.htmlFor = 'home';
            let textarea = createTextarea(markdown);
            li.appendChild(textarea);
            textarea.id = 'home';
            let button = document.createElement('button');
            li.appendChild(button);
            button.innerHTML = 'Submit';
            button.classList.add('btn');
            button.onclick = function() {
                let markdown = document.getElementById('home').value;
                clearMain();
                addLoading();
                request(
                    'PUT',
                    '/api/page/home',
                    JSON.stringify({
                        'pagename': 'home',
                        'markdown': markdown
                    }),
                    function(req) {
                        removeLoading();
                        if (req.status == 200) {
                            render(markdown);
                        } else if (req.status == 401) {
                            addHeading('Unauthorized!');
                        } else {
                            addHeading('Ridiculous!');
                        }
                    },
                    'application/json'
                );
            }
        }

        setNavItem('nav-home', '#', true);
        setNavItem('nav-tweet', '/admin/tweet/', false);
        setNavItem('nav-blog', '/admin/blog/', false);
        request('GET', '/api/page/home', '', function(req) {
            removeLoading();
            if (req.status == 200) {
                render(JSON.parse(req.responseText).markdown);
            } else {
                addHeading('Ridiculous!');
            }
        });
    }

    function hideLoadMoreOrBottomLine() {
        let loadMore = document.querySelector('#load-more');
        let bottomLine = document.querySelector('#bottom-line');
        if (loadMore) {
            loadMore.style.display = 'none';
        } else {
            bottomLine.style.display = 'none';
        }
    }

    function showLoadMoreOrBottomLine() {
        let loadMore = document.querySelector('#load-more');
        let bottomLine = document.querySelector('#bottom-line');
        if (loadMore) {
            loadMore.style.display = 'block';
        } else {
            bottomLine.style.display = 'block';
        }
    }

    function adminTweet() {
        function hideTweets() {
            document.querySelector('#new-tweet').style.display = 'none';
            document.querySelector('#display-tweets').style.display = 'none';
            hideLoadMoreOrBottomLine();
        }

        function showTweets() {
            document.querySelector('#new-tweet').style.display = 'block';
            document.querySelector('#display-tweets').style.display = 'block';
            showLoadMoreOrBottomLine();
        }

        function clearEditTweet() {
            document.querySelector('#edit-tweet').innerHTML = '';
        }

        function createTweetItem(tweet) {
            let li = createLi();
            let div0 = document.createElement('div');
            div0.style.textAlign = 'right';
            li.appendChild(div0);
            let div1 = document.createElement('div');
            div1.innerHTML = marked(tweet.markdown);
            div1.id = 'markdown';
            li.appendChild(div1);
            let div2 = document.createElement('div');
            li.appendChild(div2);
            let span = document.createElement('span');
            span.innerHTML = '...';
            span.id = 'dots';
            span.onclick = function() {
                renderEditTweet(li, tweet);
            }
            div0.appendChild(span);
            let p = document.createElement('p');
            p.style.textAlign = 'right';
            p.style.fontSize = 'small';
            p.style.textDecoration = 'overline';
            p.innerHTML = formatDate(new Date(tweet.created_time + 'Z'));
            div2.appendChild(p);
            return li;
        }

        function renderNewTweet() {
            hideTweets();
            let ul = createUl();
            document.querySelector('#edit-tweet').appendChild(ul);
            let li = createLi();
            ul.appendChild(li);
            let textarea = createTextarea('');
            li.appendChild(textarea);
            let back = createButton('Back', function() {
                clearEditTweet();
                showTweets();
            });
            li.appendChild(back);
            let submit = createButton('Submit', function() {
                let markdown = textarea.value;
                clearEditTweet();
                addLoading();
                request(
                    'POST',
                    '/api/tweet/',
                    JSON.stringify({'markdown': markdown}),
                    function(req) {
                        if (req.status == 200) {
                            let meta = JSON.parse(req.responseText);
                            removeLoading();
                            let ul = document.querySelector('#display-tweets')
                                .querySelector('ul');
                            ul.insertBefore(
                                createTweetItem({
                                    'id': meta.id,
                                    'markdown': markdown,
                                    'created_time': meta.created_time
                                }),
                                ul.querySelector('li')
                            );
                            showTweets();
                        } else if (req.status == 401) {
                            clearMain();
                            addHeading('Unauthorized!');
                        } else {
                            clearMain();
                            addHeading('Ridiculous!');
                        }
                    },
                    'application/json'
                );
            });
            li.appendChild(submit);
        }

        function renderEditTweet(item, tweet) {
            hideTweets();
            let ul = createUl();
            document.querySelector('#edit-tweet').appendChild(ul);
            let li = createLi();
            ul.appendChild(li);
            let textarea = createTextarea(tweet.markdown);
            li.appendChild(textarea);
            let back = createButton('Back', function() {
                clearEditTweet();
                showTweets();
            });
            li.appendChild(back);
            let submit = createButton('Submit', function() {
                let markdown = textarea.value;
                clearEditTweet();
                addLoading();
                request(
                    'PUT',
                    '/api/tweet/' + tweet.id,
                    JSON.stringify({'markdown': markdown}),
                    function(req) {
                        if (req.status == 200) {
                            removeLoading();
                            item.querySelector('#markdown').innerHTML =
                                marked(markdown);
                            item.querySelector('#dots').onclick = function() {
                                renderEditTweet(item, {
                                    'id': tweet.id,
                                    'markdown': markdown,
                                    'created_time': tweet.created_time
                                });
                            }
                            showTweets();
                        } else if (req.status == 401) {
                            clearMain();
                            addHeading('Unauthorized!');
                        } else {
                            clearMain();
                            addHeading('Ridiculous!');
                        }
                    },
                    'application/json'
                );
            });
            li.appendChild(submit);
            let deleteButton = createButton('Delete', function() {
                ul.parentNode.removeChild(ul);
                addLoading();
                request(
                    'DELETE',
                    '/api/tweet/' + tweet.id,
                    '',
                    function(req) {
                        if (req.status == 200) {
                            removeLoading();
                            item.parentNode.removeChild(item);
                            showTweets();
                        } else if (req.status == 401) {
                            clearMain();
                            addHeading('Unauthorized!');
                        } else {
                            clearMain();
                            addHeading('Ridiculous!');
                        }
                    }
                );
            });
            deleteButton.style.color = 'red';
            li.appendChild(deleteButton);
        }

        function renderOneTweet(tweet) {
            document.querySelector('#display-tweets')
                .querySelector('ul')
                .appendChild(createTweetItem(tweet));
        }

        function render(tweets) {
            removeLoading();
            let ul = document.querySelector('#display-tweets')
                .querySelector('ul');
            tweets.forEach(renderOneTweet);
            if (tweets.length == MAX_LENGTH) {
                addLoadMore(function() {
                    getTweets(tweets[tweets.length - 1].created_time, render);
                });
            } else {
                addBottomLine();
            }
        }

        setNavItem('nav-home', '/admin/', false);
        setNavItem('nav-tweet', '#', true);
        setNavItem('nav-blog', '/admin/blog/', false);
        let main = document.querySelector('#main');
        let div1 = document.createElement('div');
        main.appendChild(div1);
        div1.id = 'new-tweet';
        div1.appendChild(createButton('New', renderNewTweet));
        let div2 = document.createElement('div');
        main.appendChild(div2);
        div2.id = 'edit-tweet';
        let div3 = document.createElement('div');
        main.appendChild(div3);
        div3.id = 'display-tweets';
        div3.appendChild(createUl());
        getTweets(getBeforeTime(), render);
    }

    function adminBlog() {
        function hideBlogs() {
            document.querySelector('#new-blog').style.display = 'none';
            document.querySelector('#display-blogs').style.display = 'none';
            hideLoadMoreOrBottomLine();
        }

        function showBlogs() {
            document.querySelector('#new-blog').style.display = 'block';
            document.querySelector('#display-blogs').style.display = 'block';
            showLoadMoreOrBottomLine();
        }

        function clearEditBlog() {
            document.querySelector('#edit-blog').innerHTML = '';
        }

        function createBlogItem(blog) {
            let li = createLi();
            let div = document.createElement('div');
            li.appendChild(div);
            div.innerHTML = marked(blog.title);
            div.onclick = function() {
                renderEditBlog(li, blog);
            }
            return li;
        }

        function renderNewBlog() {
            hideBlogs();
            let ul = createUl();
            document.querySelector('#edit-blog').appendChild(ul);
            let li = createLi();
            ul.appendChild(li);
            let input = document.createElement('input');
            li.appendChild(input);
            input.type = 'text';
            input.style.display = 'block';
            input.style.width = '100%';
            let textarea = createTextarea('');
            li.appendChild(textarea);
            let back = createButton('Back', function() {
                clearEditBlog();
                showBlogs();
            });
            li.appendChild(back);
            let submit = createButton('Submit', function() {
                let title = input.value;
                let markdown = textarea.value;
                clearEditBlog();
                addLoading();
                request(
                    'POST',
                    '/api/blog/',
                    JSON.stringify({'title': title, 'markdown': markdown}),
                    function(req) {
                        if (req.status == 200) {
                            let meta = JSON.parse(req.responseText);
                            removeLoading();
                            let ul = document.querySelector('#display-blogs')
                                .querySelector('ul');
                            ul.insertBefore(
                                createBlogItem({
                                    'id': meta.id,
                                    'title': title,
                                    'markdown': markdown,
                                    'created_time': meta.created_time
                                }),
                                ul.querySelector('li')
                            );
                            showBlogs();
                        } else if (req.status == 401) {
                            clearMain();
                            addHeading('Unauthorized!');
                        } else {
                            clearMain();
                            addHeading('Ridiculous!');
                        }
                    },
                    'application/json'
                );
            });
            li.appendChild(submit);
        }

        function renderEditBlog(item, blog) {
            hideBlogs();
            addLoading();
            request(
                'GET',
                '/api/blog/' + blog.id,
                '',
                function(req) {
                    if (req.status == 200) {
                        removeLoading();
                        blog = JSON.parse(req.responseText);
                        let ul = createUl();
                        document.querySelector('#edit-blog').appendChild(ul);
                        let li = createLi();
                        ul.appendChild(li);
                        let input = document.createElement('input');
                        li.appendChild(input);
                        input.value = blog.title;
                        input.type = 'text';
                        input.style.display = 'block';
                        input.style.width = '100%';
                        let textarea = createTextarea(blog.markdown);
                        li.appendChild(textarea);
                        let back = createButton('Back', function() {
                            clearEditBlog();
                            showBlogs();
                        });
                        li.appendChild(back);
                        let submit = createButton('Submit', function () {
                            let title = input.value;
                            let markdown = textarea.value;
                            clearEditBlog();
                            addLoading();
                            request(
                                'PUT',
                                '/api/blog/' + blog.id,
                                JSON.stringify({'title': title, 'markdown': markdown}),
                                function(req) {
                                    if (req.status == 200) {
                                        removeLoading();
                                        let div = item.querySelector('div');
                                        div.innerHTML = marked(title);
                                        div.onclick = function() {
                                            renderEditBlog(item, {
                                                'id': blog.id,
                                                'title': title,
                                                'markdown': markdown,
                                                'created_time': blog.created_time
                                            });
                                        }
                                        showBlogs();
                                    } else if (req.status == 401) {
                                        clearMain();
                                        addHeading('Unauthorized!');
                                    } else {
                                        clearMain();
                                        addHeading('Ridiculous!');
                                    }
                                },
                                'application/json'
                            );
                        });
                        li.appendChild(submit);
                        let deleteButton = createButton('Delete', function() {
                            ul.parentNode.removeChild(ul);
                            addLoading();
                            request(
                                'DELETE',
                                '/api/blog/' + blog.id,
                                '',
                                function(req) {
                                    if (req.status == 200) {
                                        removeLoading();
                                        item.parentNode.removeChild(item);
                                        showBlogs();
                                    } else if (req.status == 401) {
                                        clearMain();
                                        addHeading('Unauthorized!');
                                    } else {
                                        addHeading('Ridiculous!');
                                    }
                                }
                            );
                        });
                        li.appendChild(deleteButton);
                        deleteButton.style.color = 'red';
                    } else {
                        clearMain();
                        addHeading('Ridiculous!');
                    }
                }
            );
        }

        function renderOneBlog(blog) {
            document.querySelector('#display-blogs')
                .querySelector('ul')
                .appendChild(createBlogItem(blog));
        }

        function render(blogs) {
            removeLoading();
            let ul = document.querySelector('#display-blogs')
                .querySelector('ul');
            blogs.forEach(renderOneBlog);
            if (blogs.length == MAX_LENGTH) {
                addLoadMore(function() {
                    getBlogs(blogs[blogs.length - 1].created_time, render);
                });
            } else {
                addBottomLine();
            }
        }

        setNavItem('nav-home', '/admin/', false);
        setNavItem('nav-tweet', '/admin/tweet/', false);
        setNavItem('nav-blog', '#', true);
        let main = document.querySelector('#main');
        let div1 = document.createElement('div');
        main.appendChild(div1);
        div1.id = 'new-blog';
        div1.appendChild(createButton('New', renderNewBlog));
        let div2 = document.createElement('div');
        main.appendChild(div2);
        div2.id = 'edit-blog';
        let div3 = document.createElement('div');
        main.appendChild(div3);
        div3.id = 'display-blogs';
        div3.appendChild(createUl());
        getBlogs(getBeforeTime(), render);
    }

    function main() {
        addLoading();
        let pathname = window.location.pathname;
        if (pathname == '/') {
            home();
        } else if (pathname == '/tweet/') {
            tweet();
        } else if (pathname == '/blog/') {
            blog();
        } else if (pathname.startsWith('/blog/')) {
            blogDetail(pathname.substring(6));
        } else if (pathname == '/login') {
            login();
        } else if (pathname == '/logout') {
            logout();
        } else if (pathname == '/admin/') {
            admin();
        } else if (pathname == '/admin/tweet/') {
            adminTweet();
        } else if (pathname == '/admin/blog/') {
            adminBlog();
        } else {
            removeLoading();
            addHeading('404');
        }
    }

    main();
})();
