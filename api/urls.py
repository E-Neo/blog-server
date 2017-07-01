from django.conf.urls import url
from .views import APITest, APIBlog, APIBlogDetail, APITweet


urlpatterns = [
    url(r'^test$', APITest.as_view(), name='api-test'),
    url(r'^blog/$', APIBlog.as_view(), name='api-blog'),
    url(r'^blog/(?P<id>[a-z0-9-]+)$', APIBlogDetail.as_view(),
        name='api-blog-detail'),
    url(r'^tweet/$', APITweet.as_view(), name='api-tweet'),
]
