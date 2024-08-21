'use client';
import { FC, useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

interface useGetSurahTranslationProps {
    id: string;
};

export const useGetSurahTranslation: FC<useGetSurahTranslationProps> = ({ id }) => {
    const [surahData, setSurahData] = useState([] as any);

    useEffect(() => {
        const getSurahTranslation = async () => {
            await invoke<Array<any>>('get_surah_translation', { id: id })
                .then(result => setSurahData(result))
                .catch(console.error);
        };

        getSurahTranslation();
    }, [id]);

    return surahData;
};