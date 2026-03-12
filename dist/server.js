// TechScript Web Auto-Generated Backend Server
const http = require('http');
const fs = require('fs');
const path = require('path');

const PORT = process.env.PORT || 3000;
const PUBLIC_DIR = path.join(__dirname, 'public');

// API Routes Definition
const apiRoutes = {};
apiRoutes['/hello'] = (req, res) => {
    res.setHeader('Content-Type', 'application/json');
    res.end(JSON.stringify({ status: 'ok', route: '/hello' }));
};

const server = http.createServer((req, res) => {
    // Very basic router
    if (req.url.startsWith('/api/')) {
        const handler = apiRoutes[req.url.replace('/api', '')];
        if (handler) return handler(req, res);
        res.writeHead(404);
        return res.end(JSON.stringify({ error: 'Not Found' }));
    }
    // Static file serving
    let filePath = path.join(PUBLIC_DIR, req.url === '/' ? 'index.html' : req.url);
    let extname = path.extname(filePath);
    let contentType = 'text/html';
    switch (extname) {
        case '.js': contentType = 'text/javascript'; break;
        case '.css': contentType = 'text/css'; break;
        case '.json': contentType = 'application/json'; break;
    }
    fs.readFile(filePath, (err, content) => {
        if (err) {
            res.writeHead(404);
            res.end('File Not Found');
        } else {
            res.writeHead(200, { 'Content-Type': contentType });
            res.end(content, 'utf-8');
        }
    });
});

server.listen(PORT, () => console.log(`TechScript dev server running on http://localhost:${PORT}`));