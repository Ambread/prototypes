import type { NextPage } from 'next';
import { trpc, useMount } from '../utils/hooks';
import { useLocalStorage } from '@mantine/hooks';
import { Messages } from '../components/Messages';
import { TextInput } from '@mantine/core';

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
        <>
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
            {login.data ? <Messages /> : <p>Logged out</p>}
        </>
    );
};

export default Home;
