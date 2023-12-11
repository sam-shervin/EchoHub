'use client';

import { ReactNode } from 'react';
import { RecoilRoot } from 'recoil';

type Props = {
    children: ReactNode;
};

export default function RecoilProvider({ children }: Props): ReactNode {
    return <RecoilRoot>{children}</RecoilRoot>;
}
