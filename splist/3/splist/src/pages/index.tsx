import type { NextPage } from 'next';
import { useEffect, useState } from 'react';
import { trpc } from '../utils/trpc';

const Home: NextPage = () => {
    const send = trpc.useMutation(['send']);
    const clear = trpc.useMutation(['clear']);

    const messagesQuery = trpc.useQuery(['messages']);
    const [messages, setMessages] = useState(() => messagesQuery.data ?? []);

    useEffect(() => {
        setMessages(messagesQuery.data ?? []);
    }, [messagesQuery.data]);

    trpc.useSubscription(['onSend'], {
        onNext(data) {
            console.log('onSend');
            setMessages((messages) => [...messages, data]);
        },
    });

    trpc.useSubscription(['onClear'], {
        onNext() {
            console.log('onClear');
            setMessages([]);
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
                {messages.map(({ id, content }) => (
                    <li key={id}>{content}</li>
                ))}
            </ul>
            <button onClick={() => clear.mutate()}>Clear</button>
        </div>
    );
};

export default Home;
