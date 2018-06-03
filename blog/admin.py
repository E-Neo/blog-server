from django.contrib import admin
from .models import Blog


@admin.register(Blog)
class TweetAdmin(admin.ModelAdmin):
    list_display = ('title', 'publish_time')
