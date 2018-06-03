import uuid
from django.db import models


class Blog(models.Model):
    id = models.UUIDField(primary_key=True, default=uuid.uuid4,
                          editable=False)
    title = models.CharField(max_length=128)
    publish_time = models.DateTimeField(auto_now_add=True)
    content = models.TextField()

    def __str__(self):
        return self.id

    class Meta:
        ordering = ['-publish_time']
