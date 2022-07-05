import type { NextPage } from 'next';
import { trpc } from '../utils/trpc';

const Home: NextPage = () => {
    const messages = trpc.useQuery(['messages']);

    if (!messages.data) {
        return <h1>Loading...</h1>;
    }

    return (
        <ul>
            {messages.data.map(({ id, content }) => (
                <li key={id}>{content}</li>
            ))}
        </ul>
    );
};

export default Home;
