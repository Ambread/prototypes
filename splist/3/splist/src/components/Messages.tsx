import { Badge, Button, Divider, TextInput, Text } from '@mantine/core';
import { FC } from 'react';
import { useEffect, useState } from 'react';
import styled from 'styled-components';
import { trpc } from '../utils/hooks';

const Main = styled.main`
    grid-area: main;
    overflow-y: scroll;
    scroll-snap-type: y proximity;
    display: flex;
    flex-flow: column;

    > :first-child {
        margin-top: auto;
    }

    > :last-child {
        scroll-snap-align: end;
    }
`;

const Footer = styled.footer`
    grid-area: footer;
    border-top: solid thin grey;

    padding: 1em;
    display: flex;
    align-items: end;
    gap: 1em;
`;

export interface Props {
    channelId: string;
}

export const Messages: FC<Props> = ({ channelId }) => {
    const send = trpc.useMutation(['text.send']);
    const clear = trpc.useMutation(['text.clear']);

    const messagesQuery = trpc.useQuery(['text.messages', { channelId }]);
    const [messages, setMessages] = useState(() => messagesQuery.data ?? []);

    useEffect(() => {
        setMessages(messagesQuery.data ?? []);
    }, [messagesQuery.data]);

    trpc.useSubscription(['text.onSend'], {
        onNext(data) {
            console.log('onSend');
            setMessages((messages) => [...messages, data]);
        },
    });

    trpc.useSubscription(['text.onClear'], {
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
            <Main>
                {messages.map(({ id, content, author }) => (
                    <div key={id}>
                        <Divider p={10} />
                        <Badge p={10}>{author.name}</Badge>
                        <Text p={10}>{content}</Text>
                    </div>
                ))}
            </Main>
            <Footer>
                <TextInput
                    style={{ flexGrow: 1 }}
                    placeholder="Message"
                    value={content}
                    onChange={(e) => setContent(e.target.value)}
                    onKeyDown={(e) => {
                        if (e.key !== 'Enter') {
                            return;
                        }
                        send.mutate({ content, channelId });
                        setContent('');
                    }}
                />
                <Button onClick={() => clear.mutate()}>Clear All</Button>
            </Footer>
        </>
    );
};
