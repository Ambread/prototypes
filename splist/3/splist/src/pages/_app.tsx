import { withTRPC } from '@trpc/next';
import type { AppProps } from 'next/app';
import { AppRouter } from '../server';
import { wsLink, createWSClient } from '@trpc/client/links/wsLink';
import { httpLink } from '@trpc/client/links/httpLink';

function App({ Component, pageProps }: AppProps) {
    return <Component {...pageProps} />;
}

export default withTRPC<AppRouter>({
    config({ ctx }) {
        const links = [];

        if (typeof window === 'undefined') {
            links.push(httpLink({ url: 'http://localhost:3000' }));
        } else {
            const client = createWSClient({
                url: 'ws://localhost:3001',
            });
            links.push(wsLink({ client }));
        }

        return { links };
    },
})(App);
