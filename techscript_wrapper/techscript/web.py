"""TechScript Native Web Standard Library

Provides the ``WebPage`` class injected into the environment when ``use web`` is executed.
This allows TechScript to build full UIs purely natively and serve them instantly.
"""
import http.server
import socketserver
import threading
import webbrowser
import os
import json
import tempfile
from techscript.errors import TechScriptError

class WebPageNative:
    def __init__(self, title: str = "TechScript App"):
        self.title = title
        self._styles = []
        self._scripts = []
        self._body_elements = []

    def style(self, selector: str, rules: dict) -> 'WebPageNative':
        """Compile a dict of styles into CSS."""
        css_rules = "; ".join(f"{k}: {v}" for k, v in rules.items())
        self._styles.append(f"{selector} {{ {css_rules} }}")
        return self

    def script(self, code: str) -> 'WebPageNative':
        self._scripts.append(code)
        return self

    def _element(self, tag: str, content: str | list = "", attrs: dict = None) -> str:
        attr_str = ""
        if attrs:
            attr_str = " " + " ".join(f'{k}="{v}"' for k, v in attrs.items())
        
        inner = ""
        if isinstance(content, list):
            inner = "\n".join(str(c) for c in content)
        else:
            inner = str(content)
            
        return f"<{tag}{attr_str}>{inner}</{tag}>"

    # -- Common HTML Builders --
    
    def h1(self, content: str, attrs: dict = None) -> str: return self._element("h1", content, attrs)
    def h2(self, content: str, attrs: dict = None) -> str: return self._element("h2", content, attrs)
    def h3(self, content: str, attrs: dict = None) -> str: return self._element("h3", content, attrs)
    def p(self, content: str, attrs: dict = None) -> str: return self._element("p", content, attrs)
    def div(self, content: list | str, attrs: dict = None) -> str: return self._element("div", content, attrs)
    def span(self, content: str, attrs: dict = None) -> str: return self._element("span", content, attrs)
    def button(self, content: str, attrs: dict = None) -> str: return self._element("button", content, attrs)
    def input(self, attrs: dict = None) -> str: return self._element("input", "", attrs).replace("></input>", "/>")
    def img(self, attrs: dict = None) -> str: return self._element("img", "", attrs).replace("></img>", "/>")
    
    def raw(self, content: str) -> str:
        """Allow raw HTML ingestion if necessary."""
        return str(content)

    def body(self, elements: list | str) -> 'WebPageNative':
        if isinstance(elements, list):
            self._body_elements.extend(elements)
        else:
            self._body_elements.append(str(elements))
        return self

    def render(self) -> str:
        """Compose the final HTML document."""
        styles = "\n".join(self._styles)
        scripts = "\n".join(self._scripts)
        body_content = "\n".join(self._body_elements)
        
        return f'''<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{self.title}</title>
    <style>
{styles}
    </style>
</head>
<body>
    <div id="app">
{body_content}
    </div>
    <script>
{scripts}
    </script>
</body>
</html>'''

    def run(self, port: int = 8080):
        """Starts a local server and opens the page in the browser."""
        html_content = self.render()
        
        # We serve out of a temporary directory
        temp_dir = tempfile.mkdtemp(prefix="techscript_web_")
        index_path = os.path.join(temp_dir, "index.html")
        
        with open(index_path, "w", encoding="utf-8") as f:
            f.write(html_content)

        class Handler(http.server.SimpleHTTPRequestHandler):
            def __init__(self, *args, **kwargs):
                super().__init__(*args, directory=temp_dir, **kwargs)
            def log_message(self, format, *args):
                pass # Suppress default logging

        import socket
        
        # Auto-select a free port if the default is busy
        if port != 0:
            test_sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            test_sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
            result = test_sock.connect_ex(("localhost", port))
            test_sock.close()
            if result == 0:
                # Port in use — pick a random free one
                free_sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
                free_sock.bind(("", 0))
                port = free_sock.getsockname()[1]
                free_sock.close()
        
        class ReusableTCPServer(socketserver.TCPServer):
            allow_reuse_address = True
        
        try:
            httpd = ReusableTCPServer(("", port), Handler)
            url = f"http://localhost:{port}"
            print(f"Server started at {url}")
            print(f"Press Ctrl+C to stop.")
            
            # Open browser after a short delay (non-blocking)
            threading.Timer(0.5, lambda: webbrowser.open(url)).start()
            
            httpd.serve_forever()
        except OSError as e:
            raise TechScriptError(f"Failed to start server: {e}")
        except KeyboardInterrupt:
            print("\nShutting down server.")
            httpd.server_close()
