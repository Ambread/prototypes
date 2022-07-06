import { withTRPC } from '@trpc/next';
import type { AppProps } from 'next/app';
import type { AppRouter } from '../server';
import { wsLink, createWSClient } from '@trpc/client/links/wsLink';
import { TRPCLink } from '@trpc/client';

function App({ Component, pageProps }: AppProps) {
    return (
        <>
            <Component {...pageProps} />
            <style>
                {`
                    body {
                        margin: 0;
                    }
                `}
            </style>
        </>
    );
}

const links = [] as TRPCLink<AppRouter>[];

if (typeof window !== 'undefined') {
    const client = createWSClient({
        url: 'ws://localhost:3001',
    });
    links.push(wsLink({ client }));
}

export default withTRPC<AppRouter>({
    config() {
        return {
            links,
        };
    },
})(App);
