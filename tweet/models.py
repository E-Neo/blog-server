import uuid
from django.db import models


class Tweet(models.Model):
    id = models.UUIDField(primary_key=True, default=uuid.uuid4,
                          editable=False)
    publish_time = models.DateTimeField(auto_now_add=True)
    content = models.CharField(max_length=512)

    def __str__(self):
        return self.id

    class Meta:
        ordering = ['-publish_time']
