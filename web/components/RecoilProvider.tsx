'use client';

import type { PropsWithChildren, ReactNode } from 'react';
import { RecoilRoot } from 'recoil';

type Props = PropsWithChildren<{}>;

export default function RecoilProvider({ children }: Props): ReactNode {
    return <RecoilRoot>{children}</RecoilRoot>;
}
