import os
import dj_database_url
from pathlib import Path


PROJECT_PACKAGE = Path(__file__).resolve().parent


# The full path to the repository root.

BASE_DIR = PROJECT_PACKAGE.parent


data_dir_key = 'DJANGOPROJECT_DATA_DIR'
DATA_DIR = Path(os.environ[data_dir_key]) if data_dir_key in os.environ \
           else BASE_DIR.parent

secret_key = os.environ['SECRET_KEY']
SECRETS = {'secret_key': secret_key}
SECRET_KEY = str(SECRETS['secret_key'])

# SECURITY WARNING: don't run with debug turned on in production!
DEBUG = False

ALLOWED_HOSTS = ['localhost', 'e-neo.herokuapp.com']


# Application definition

INSTALLED_APPS = [
    'api.apps.ApiConfig',
    'blog.apps.BlogConfig',
    'page.apps.PageConfig',
    'tweet.apps.TweetConfig',

    'django.contrib.admin',
    'django.contrib.auth',
    'django.contrib.contenttypes',
    'django.contrib.sessions',
    'django.contrib.messages',
    'django.contrib.staticfiles',

    'rest_framework',
]

REST_FRAMEWORK = {
    'DEFAULT_AUTHENTICATION_CLASSES': (
        'rest_framework.authentication.SessionAuthentication',
        'rest_framework.authentication.TokenAuthentication',
    )
}

MIDDLEWARE = [
    'django.middleware.security.SecurityMiddleware',
    'whitenoise.middleware.WhiteNoiseMiddleware',
    'django.contrib.sessions.middleware.SessionMiddleware',
    'django.middleware.common.CommonMiddleware',
    'django.middleware.csrf.CsrfViewMiddleware',
    'django.contrib.auth.middleware.AuthenticationMiddleware',
    'django.contrib.messages.middleware.MessageMiddleware',
    'django.middleware.clickjacking.XFrameOptionsMiddleware',
]

ROOT_URLCONF = 'zone.urls'

TEMPLATES = [
    {
        'BACKEND': 'django.template.backends.django.DjangoTemplates',
        'DIRS': [str(PROJECT_PACKAGE.joinpath('templates'))],
        'APP_DIRS': True,
        'OPTIONS': {
            'context_processors': [
                'django.template.context_processors.debug',
                'django.template.context_processors.request',
                'django.contrib.auth.context_processors.auth',
                'django.contrib.messages.context_processors.messages',
            ],
        },
    },
]

WSGI_APPLICATION = 'zone.wsgi.application'


# Database

DATABASES = {
    'default': {
        'ENGINE': 'django.db.backends.postgresql',
        'NAME': 'zone',
        'USER': 'e-neo',
        'HOST': SECRETS.get('db_host', ''),
        'PASSWORD': SECRETS.get('db_password', '')
    }
}
db_from_env = dj_database_url.config()
DATABASES['default'].update(db_from_env)

# Password validation
# https://docs.djangoproject.com/en/1.11/ref/settings/#auth-password-validators

AUTH_PASSWORD_VALIDATORS = [
    {
        'NAME': 'django.contrib.auth.password_validation.UserAttributeSimilarityValidator',
    },
    {
        'NAME': 'django.contrib.auth.password_validation.MinimumLengthValidator',
    },
    {
        'NAME': 'django.contrib.auth.password_validation.CommonPasswordValidator',
    },
    {
        'NAME': 'django.contrib.auth.password_validation.NumericPasswordValidator',
    },
]


# Internationalization
# https://docs.djangoproject.com/en/1.11/topics/i18n/

LANGUAGE_CODE = 'en-us'

TIME_ZONE = 'Asia/Shanghai'

USE_I18N = True

USE_L10N = True

USE_TZ = True


# Static files (CSS, JavaScript, Images)

STATIC_URL = '/static/'
STATIC_ROOT = str(BASE_DIR.joinpath('static'))
STATICFILES_DIRS = [str(PROJECT_PACKAGE.joinpath('static'))]
STATICFILES_STORAGE = 'whitenoise.django.GzipManifestStaticFilesStorage'
