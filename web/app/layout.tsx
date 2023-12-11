import type { Metadata } from 'next';
import type { ReactNode } from 'react';
import RecoilProvider from '@components/RecoilProvider';

export const metadata: Metadata = {
    title: 'EchoHub | Home',
    description: 'EchoHub is a Discord-inspired chat app.',
    keywords: 'echohub, chat, discord, clone',
    creator: 'The EchoHub Team',
};

type Props = {
    children: ReactNode;
};

export default function RootLayout({ children }: Props): ReactNode {
    return (
        <html lang='en-US'>
            <body>
                <RecoilProvider>{children}</RecoilProvider>
            </body>
        </html>
    );
}
