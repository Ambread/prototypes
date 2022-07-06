import { FC } from 'react';
import { useEffect, useState } from 'react';
import { trpc } from '../utils/hooks';
import { useLocalStorage } from '@mantine/hooks';

export const Messages: FC = () => {
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
                placeholder="Message"
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
                {messages.map(({ id, content, author }) => (
                    <li key={id}>
                        [{author.name}] {content}
                    </li>
                ))}
            </ul>
            <button onClick={() => clear.mutate()}>Clear</button>
        </div>
    );
};
