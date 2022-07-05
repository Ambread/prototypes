import type { NextPage } from 'next';
import { useState } from 'react';
import { trpc } from '../utils/trpc';

const Home: NextPage = () => {
    const utils = trpc.useContext();
    const messages = trpc.useQuery(['messages']);
    const send = trpc.useMutation(['send']);
    const clear = trpc.useMutation(['clear']);
    trpc.useSubscription(['onSend'], {
        onNext(data) {
            console.log('onSent');
            messages.data?.push(data);
        },
    });

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
                onKeyDown={(e) => {
                    if (e.key !== 'Enter') {
                        return;
                    }
                    send.mutate({ content });
                    setContent('');
                }}
            />
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
