from rest_framework import serializers
from rest_framework.response import Response
from rest_framework.views import APIView
from blog.models import Blog
from tweet.models import Tweet


class APITest(APIView):
    def get(self, request, format=None):
        content = {'data': 'E-Neo says hello.'}
        return Response(content)


class APIBlogSerializer(serializers.ModelSerializer):
    class Meta:
        model = Blog
        fields = ('id', 'publish_time', 'title')


class APIBlog(APIView):
    def get(self, request):
        try:
            time_before = request.GET.get('time_before')
            if time_before is None:
                queryset = Blog.objects.order_by('-publish_time')[:10]
                serializer = APIBlogSerializer(queryset, many=True)
                return Response(serializer.data)
            else:
                queryset = Blog.objects.filter(publish_time__lt=time_before)
                queryset = queryset.order_by('-publish_time')[:10]
                serializer = APIBlogSerializer(queryset, many=True)
                return Response(serializer.data)
        except Exception as e:
            return Response(status=400)


class APIBlogDetailSerializer(serializers.ModelSerializer):
    class Meta:
        model = Blog
        fields = ('id', 'publish_time', 'title', 'content')


class APIBlogDetail(APIView):
    def get(self, request, id=None):
        try:
            queryset = Blog.objects.get(id=id)
            serializer = APIBlogDetailSerializer(queryset)
            return Response(serializer.data)
        except Exception as e:
            return Response(status=404)


class APITweetSerializer(serializers.ModelSerializer):
    class Meta:
        model = Tweet
        fields = ('id', 'publish_time', 'content')


class APITweet(APIView):
    def get(self, request):
        try:
            time_before = request.GET.get('time_before')
            if time_before is None:
                queryset = Tweet.objects.order_by('-publish_time')[:10]
                serializer = APITweetSerializer(queryset, many=True)
                return Response(serializer.data)
            else:
                queryset = Tweet.objects.filter(publish_time__lt=time_before)
                queryset = queryset.order_by('-publish_time')[:10]
                serializer = APITweetSerializer(queryset, many=True)
                return Response(serializer.data)
        except Exception as e:
            return Response(status=400)
