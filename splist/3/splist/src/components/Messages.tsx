import {
    Badge,
    Button,
    Divider,
    Group,
    Stack,
    TextInput,
    Text,
    Paper,
    ScrollArea,
    Footer,
} from '@mantine/core';
import { FC } from 'react';
import { useEffect, useState } from 'react';
import { trpc } from '../utils/hooks';

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
        <>
            {messages.map(({ id, content, author }) => (
                <Paper key={id}>
                    <Divider p={10} />
                    <Badge p={10}>{author.name}</Badge>
                    <Text p={10}>{content}</Text>
                </Paper>
            ))}

            <Group>
                <TextInput
                    style={{ flexGrow: 1 }}
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
                <Button onClick={() => clear.mutate()}>Clear All</Button>
            </Group>
        </>
    );
};
