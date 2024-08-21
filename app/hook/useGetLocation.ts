'use client';
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

export const useGetLocation = () => {
    const [myLocation, setMyLocation] = useState([] as any);

    useEffect(() => {
        const getLocation = async () => {
            invoke<any>('get_location')
                .then(result => setMyLocation(result))
                .catch(console.error);
        };

        getLocation();
    }, []);

    return myLocation;
};