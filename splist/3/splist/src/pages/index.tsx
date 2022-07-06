import type { NextPage } from 'next';
import { trpc, useMount } from '../utils/hooks';
import { useLocalStorage } from '@mantine/hooks';
import { Messages } from '../components/Messages';
import { TextInput, Title, Text, Alert } from '@mantine/core';
import { AlertCircle } from 'tabler-icons-react';
import styled from 'styled-components';

const Grid = styled.div`
    display: grid;
    width: 100vw;
    height: 100vh;
    grid-template:
        'header header' 10vh
        'nav    main  ' 1fr
        'user   footer' 10vh
        / 30vh 1fr;
`;

const Header = styled.header`
    grid-area: header;
    border-bottom: solid thin grey;
    display: flex;
    align-items: center;
    padding: 2em;
`;

const Nav = styled.nav`
    grid-area: nav;
    border-right: solid thin grey;
    padding: 3em;
    display: flex;
    flex-direction: column;
    align-items: center;
`;

const Aside = styled.aside`
    grid-area: user;
    border-right: solid thin grey;
    border-top: solid thin grey;
    padding: 1em;
`;

const Home: NextPage = () => {
    const [name, setName] = useLocalStorage({
        key: 'name',
        defaultValue: '',
    });

    const login = trpc.useMutation(['login']);

    useMount(() => {
        login.mutate({ name });
    });

    return (
        <Grid>
            <Header>
                <Title>Splist</Title>
            </Header>
            <Nav>
                <Text>Cool navbar</Text>
            </Nav>
            <Aside>
                <TextInput
                    label="Username"
                    value={name}
                    onChange={(e) => {
                        setName(e.target.value);
                    }}
                    onKeyDown={(e) => {
                        if (e.key !== 'Enter') {
                            return;
                        }
                        login.mutate({ name });
                    }}
                />
            </Aside>
            {login.data ? (
                <Messages />
            ) : (
                <Alert
                    icon={<AlertCircle size={16} />}
                    color="red"
                    title="Unauthorized"
                >
                    You need to log in
                </Alert>
            )}
        </Grid>
    );
};

export default Home;
