import { URL } from 'url';
import { Client } from './client';

const client = new Client(new URL('http://localhost:3000'));
client.testRest();
