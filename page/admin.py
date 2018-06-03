from django.contrib import admin
from .models import Page, Vocabulary


@admin.register(Page)
class PageAdmin(admin.ModelAdmin):
    list_display = ('name',)


@admin.register(Vocabulary)
class VocabularyAdmin(admin.ModelAdmin):
    list_display = ('word', 'datetime', 'status', 'info')
