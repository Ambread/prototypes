import { Server } from 'ws';
import { applyWSSHandler } from '@trpc/server/adapters/ws';
import { appRouter } from '..';
import { createContext } from '../createContext';

const port = 3001;

const wss = new Server({ port });
const handler = applyWSSHandler({ wss, router: appRouter, createContext });

wss.on('connection', (ws) => {
    console.log(`++ Connection (${wss.clients.size})`);
    ws.once('close', () => {
        console.log(`-- Connection (${wss.clients.size})`);
    });
});
console.log(`✅ WebSocket Server listening on ws://localhost:${port}`);

process.on('SIGTERM', () => {
    console.log('SIGTERM');
    handler.broadcastReconnectNotification();
    wss.close();
});
