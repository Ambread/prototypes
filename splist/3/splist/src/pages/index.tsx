import type { NextPage } from 'next';
import { useState } from 'react';
import { trpc } from '../utils/trpc';

const Home: NextPage = () => {
    const messages = trpc.useQuery(['messages']);
    const send = trpc.useMutation(['send']);
    const clear = trpc.useMutation(['clear']);

    const [content, setContent] = useState('');

    if (!messages.data) {
        return <h1>Loading...</h1>;
    }

    return (
        <div>
            <input
                type="text"
                value={content}
                onChange={(e) => setContent(e.target.value)}
            />
            <button onClick={() => send.mutate({ content })}>Send</button>
            <ul>
                {messages.data.map(({ id, content }) => (
                    <li key={id}>{content}</li>
                ))}
            </ul>
            <button onClick={() => clear.mutate()}>Clear</button>
        </div>
    );
};

export default Home;
