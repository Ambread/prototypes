import { EventEmitter } from 'events';
import { WebSocket } from 'ws';
import { request } from 'undici';
import { URL } from 'url';
import { HttpMethod, RequestOptions } from 'undici/types/dispatcher';

export class Client {
    private rest: Rest;
    private gateway: Gateway;

    constructor(url: URL) {
        this.rest = new Rest(url);
        this.gateway = new Gateway(url);
    }

    public testRest() {
        this.rest.testRest();
    }
}

class Rest {
    constructor(private baseUrl: URL) {}

    public async fetch(path: string, method: HttpMethod) {
        const url = new URL(path, this.baseUrl);
        const response = await request(url, { method });
        return response.body.json();
    }

    public get(path: string) {
        return this.fetch(path, 'GET');
    }

    public async testRest() {
        console.log(await this.get(''));
    }
}

class Gateway {
    private ws: WebSocket;

    constructor(baseUrl: URL) {
        const url = new URL('./ws', baseUrl);
        url.protocol = 'ws';

        this.ws = new WebSocket(url);

        this.ws.on('open', () => {
            this.ws.send('wew');
        });
        this.ws.on('message', (data) => {
            console.log(data.toString());
        });
    }
}
