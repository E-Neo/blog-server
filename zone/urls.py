from django.conf.urls import include, url
from django.contrib import admin
from django.views.generic import TemplateView

from page import views as page_views


urlpatterns = [
    url(r'^$', page_views.home),
    url(r'^api/', include('api.urls')),
    url(r'^blog/$', TemplateView.as_view(template_name='blog.html')),
    url(r'^blog/(?P<id>[a-z0-9-]+)$',
        TemplateView.as_view(template_name='blog_detail.html')),
    url(r'^tweet/$', TemplateView.as_view(template_name='tweet.html')),
    url(r'^admin/', admin.site.urls),
]
