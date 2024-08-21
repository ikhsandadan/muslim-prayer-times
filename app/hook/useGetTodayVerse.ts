'use client';
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

export const useGetTodayVerse = () => {
    const [todayVerse, setTodayVerse] = useState([] as any);

    useEffect(() => {
        const getRandomVerse = async () => {
            await invoke<any>('get_random_verse')
                .then(result => setTodayVerse(result))
                .catch(console.error);
        };

        getRandomVerse();
    }, []);

    return todayVerse;
};