import { createServer } from "node:http";
import { createReadStream } from "node:fs";
import { stat } from "node:fs/promises";
import { dirname, extname, join, resolve, sep } from "node:path";
import { fileURLToPath } from "node:url";

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..", "..");
const publicRoot = resolve(
    process.env.PREVIEW_ROOT ?? join(repositoryRoot, "target", "dx", "puddle", "release", "web", "public"),
);
const host = process.env.PREVIEW_HOST ?? "127.0.0.1";
const port = Number(process.env.PREVIEW_PORT ?? 4173);
const basePath = "/puddle";

const contentTypes = {
    ".css": "text/css; charset=utf-8",
    ".html": "text/html; charset=utf-8",
    ".ico": "image/x-icon",
    ".js": "text/javascript; charset=utf-8",
    ".json": "application/json; charset=utf-8",
    ".svg": "image/svg+xml",
    ".wasm": "application/wasm",
};

function send(response, status, headers, body = "") {
    response.writeHead(status, headers);
    response.end(body);
}

function isInsidePublicRoot(filePath) {
    return filePath === publicRoot || filePath.startsWith(`${publicRoot}${sep}`);
}

async function serve(request, response) {
    if (request.method !== "GET" && request.method !== "HEAD") {
        send(response, 405, { Allow: "GET, HEAD" }, "Method Not Allowed\n");
        return;
    }

    let url;
    try {
        url = new URL(request.url ?? "/", `http://${host}:${port}`);
    } catch {
        send(response, 400, { "Content-Type": "text/plain; charset=utf-8" }, "Bad Request\n");
        return;
    }

    if (url.pathname === `${basePath}/__health`) {
        send(response, 200, { "Content-Type": "application/json; charset=utf-8", "Cache-Control": "no-store" }, '{"status":"ok"}');
        return;
    }

    if (url.pathname === basePath) {
        send(response, 308, { Location: `${basePath}/` });
        return;
    }

    if (!url.pathname.startsWith(`${basePath}/`)) {
        send(response, 404, { "Content-Type": "text/plain; charset=utf-8" }, "Not Found\n");
        return;
    }

    let requestedPath;
    try {
        requestedPath = decodeURIComponent(url.pathname.slice(basePath.length + 1));
    } catch {
        send(response, 400, { "Content-Type": "text/plain; charset=utf-8" }, "Bad Request\n");
        return;
    }

    const filePath = resolve(publicRoot, requestedPath || "index.html");
    if (!isInsidePublicRoot(filePath)) {
        send(response, 403, { "Content-Type": "text/plain; charset=utf-8" }, "Forbidden\n");
        return;
    }

    let file;
    try {
        file = await stat(filePath);
    } catch {
        send(response, 404, { "Content-Type": "text/plain; charset=utf-8" }, "Not Found\n");
        return;
    }

    if (!file.isFile()) {
        send(response, 404, { "Content-Type": "text/plain; charset=utf-8" }, "Not Found\n");
        return;
    }

    const contentType = contentTypes[extname(filePath).toLowerCase()] ?? "application/octet-stream";
    response.writeHead(200, {
        "Cache-Control": "no-store",
        "Content-Length": file.size,
        "Content-Type": contentType,
    });

    if (request.method === "HEAD") {
        response.end();
        return;
    }

    createReadStream(filePath).pipe(response);
}

const server = createServer((request, response) => {
    serve(request, response).catch(() => {
        if (!response.headersSent) {
            send(response, 500, { "Content-Type": "text/plain; charset=utf-8" }, "Internal Server Error\n");
        } else {
            response.destroy();
        }
    });
});

server.on("error", (error) => {
    console.error(error);
    process.exitCode = 1;
});

server.listen(port, host, () => {
    console.log(`Puddle preview server listening at http://${host}:${port}${basePath}/`);
});

function close() {
    server.close(() => process.exit(0));
}

process.once("SIGINT", close);
process.once("SIGTERM", close);
