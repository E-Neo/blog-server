from django.shortcuts import render
from .models import Page


def home(request):
    try:
        page = Page.objects.get(name='home')
        content = page.content
        script = page.script
    except:
        content = ''
        script = ''
    return render(request, 'home.html',
                  {'content': content, 'script': script})
