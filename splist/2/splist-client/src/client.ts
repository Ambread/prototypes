import { EventEmitter } from 'events';
import { WebSocket } from 'ws';

class Client extends EventEmitter {
    private ws: WebSocket;

    constructor(url: string) {
        super();
        this.ws = new WebSocket(url);
        this.ws.on('open', () => {
            this.ws.send('wew');
        });
        this.ws.on('message', (data) => {
            console.log(data);
        });
    }
}
