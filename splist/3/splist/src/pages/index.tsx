import type { NextPage } from 'next';
import { trpc, useMount } from '../utils/hooks';
import { useLocalStorage } from '@mantine/hooks';
import { Messages } from '../components/Messages';
import { TextInput, Title, Text, Alert } from '@mantine/core';
import { AlertCircle } from 'tabler-icons-react';

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
        <div
            style={{
                display: 'grid',
                width: '100vw',
                height: '100vh',
                gridTemplate: `
                    "header header" 10vh
                    "nav    main  " 1fr
                    "user   footer" 10vh
                    / 30vh 1fr
                `,
            }}
        >
            <header
                style={{
                    gridArea: 'header',
                    borderBottom: 'solid thin grey',
                    display: 'flex',
                    alignItems: 'center',
                    padding: '3em',
                }}
            >
                <Title>Splist</Title>
            </header>
            <nav
                style={{
                    gridArea: 'nav',
                    borderRight: 'solid thin grey',
                    padding: '3em',
                    display: 'flex',
                    flexDirection: 'column',
                    alignItems: 'center',
                }}
            >
                <Text>Cool navbar</Text>
            </nav>
            <aside
                style={{
                    gridArea: 'user',
                    borderRight: 'solid thin grey',
                    borderTop: 'solid thin grey',
                    padding: '1em',
                }}
            >
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
            </aside>
            {login.data ? (
                <Messages />
            ) : (
                <>
                    <main style={{ gridArea: 'main' }}></main>
                    <footer style={{ gridArea: 'footer', padding: '1em' }}>
                        <Alert
                            icon={<AlertCircle size={16} />}
                            color="red"
                            title="Unauthorized"
                        >
                            You need to log in
                        </Alert>
                    </footer>
                </>
            )}
        </div>
    );
};

export default Home;
