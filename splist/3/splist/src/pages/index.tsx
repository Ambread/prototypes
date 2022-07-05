import type { NextPage } from 'next';
import { trpc } from '../utils/trpc';

const Home: NextPage = () => {
    const hello = trpc.useQuery(['hello', { text: 'cool person' }]);

    if (!hello.data) {
        return <h1>Loading...</h1>;
    }

    return <h1>{hello.data.greeting}</h1>;
};

export default Home;
