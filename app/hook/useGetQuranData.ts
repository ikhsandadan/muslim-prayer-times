'use client';
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

export const useGetQuranData = () => {
    const [quranData, setQuranData] = useState([] as any);

    useEffect(() => {
        const getQuranData = async () => {
            await invoke<any>('get_quran_data')
                .then(result => setQuranData(result))
                .catch(console.error);
        };

        getQuranData();
    }, []);

    return quranData;
};