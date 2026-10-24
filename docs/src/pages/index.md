{% extends "layouts/index.html" %} {% block title %}Home{% endblock title %} {% block header %}
# ![](/assets/logo.svg)

-   [Install](/install.html)
-   [GitHub](https://github.com/aleksrutins/ionian)
-   Docs
{% endblock header %} {% block content %}
Ionian is a fast and simple static site generator using [Tera](https://keats.github.io/tera/) templates.

Getting started is as simple as creating a `pages/index.html` file:

```html
<!DOCTYPE html>
<html lang="en">
    <head>
    	<meta charset="UTF-8">
    	<meta name="viewport" content="width=device-width, initial-scale=1.0">
    	<title>Hello</title>
    </head>
    <body>
        <h1>Hello World!</h1>
    </body>
</html>
```

And run `ionian build` to put a built site in `_build` — or `ionian dev` for a fast dev server.

Extract the layout into `layouts/base.html` and enable markdown by wrapping the contents in `<ionian.markdown>`:

```html
{% raw %}<!DOCTYPE html>
<html lang="en">
    <head>
    	<meta charset="UTF-8">
    	<meta name="viewport" content="width=device-width, initial-scale=1.0">
    	<title>{% block title %}{% endblock title %}</title>
    </head>
    <body>
        {% <ionian.markdown> %}
        {% block content %}{% endblock content %}
        {% </ionian.markdown> %}
    </body>
</html>
{% endraw %}
```

And simplify `index.html` into `index.md`:

```md
{% raw %}{% extends "layouts/base.html" %}
{% block title %}Hello{% endblock title %}
{% block content %}
{% <ionian.markdown> %}
# Hello World!
{% </ionian.markdown> %}
{% endblock content %}
{% endraw %}
```

(For all Tera features, the root for templates is the root directory you specify.)

Add hooks in `ionian.toml` to run external tools:

```toml
[[hooks]]
name = "tailwind"
during = "PreBuild"
dev = {Watch = ["tailwind.css"]}
command = "bun tailwindcss -i tailwind.css -o assets/app.css 2> /dev/null"
```

And read more in the documentation, which will be coming soon!
{% endblock content %}
