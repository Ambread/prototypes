import type { NextPage } from 'next';
import { useState } from 'react';
import { trpc } from '../utils/trpc';

const Home: NextPage = () => {
    const send = trpc.useMutation(['send']);
    const clear = trpc.useMutation(['clear']);

    const messagesQuery = trpc.useQuery(['messages']);
    const [messages, setMessages] = useState(() => messagesQuery.data ?? []);

    trpc.useSubscription(['onSend'], {
        onNext(data) {
            console.log('onSent');
            setMessages((messages) => [...messages, data]);
        },
    });

    const [content, setContent] = useState('');

    if (!messagesQuery.data) {
        return <h1>Loading...</h1>;
    }

    return (
        <div>
            <input
                type="text"
                value={content}
                onChange={(e) => setContent(e.target.value)}
                onKeyDown={(e) => {
                    if (e.key !== 'Enter') {
                        return;
                    }
                    send.mutate({ content });
                    setContent('');
                }}
            />
            <ul>
                {messages?.map(({ id, content }) => (
                    <li key={id}>{content}</li>
                ))}
            </ul>
            <button onClick={() => clear.mutate()}>Clear</button>
        </div>
    );
};

export default Home;
