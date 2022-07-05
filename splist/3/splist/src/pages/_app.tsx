import { withTRPC } from '@trpc/next';
import type { AppProps } from 'next/app';
import { AppRouter } from '../server';

function App({ Component, pageProps }: AppProps) {
    return <Component {...pageProps} />;
}

export default withTRPC<AppRouter>({
    config({ ctx }) {
        return { url: 'http://localhost:3000/api/trpc' };
    },
})(App);
