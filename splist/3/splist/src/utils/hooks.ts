import { createReactQueryHooks } from '@trpc/react';
import { EffectCallback, useEffect } from 'react';
import type { AppRouter } from '../server';

export const trpc = createReactQueryHooks<AppRouter>();

// eslint-disable-next-line react-hooks/exhaustive-deps
export const useMount = (callback: EffectCallback) => useEffect(callback, []);
