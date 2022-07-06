import { withTRPC } from '@trpc/next';
import type { AppProps } from 'next/app';
import type { AppRouter } from '../server';
import { wsLink, createWSClient } from '@trpc/client/links/wsLink';
import { TRPCLink } from '@trpc/client';
import { createGlobalStyle } from 'styled-components';

const GlobalStyle = createGlobalStyle`
    html, body {
        margin: 0;
        padding: 0;
    }
`;

function App({ Component, pageProps }: AppProps) {
    return (
        <>
            <Component {...pageProps} />
            <GlobalStyle />
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
