'use client';
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

export const useGetPrayerTimesThisDay = () => {
    const [myPrayerTimes, setMyPrayerTimes] = useState([] as any);

    useEffect(() => {
        const updatePrayerTimes = async () => {
            await invoke<any>('get_prayer_times_this_day')
                .then(result => setMyPrayerTimes(result))
                .catch(console.error);
        };

        updatePrayerTimes();
    }, []);

    return myPrayerTimes;
};

export const useGetNearestPrayer = () => {
    const [myPrayerTimes, setMyPrayerTimes] = useState([] as any);

    useEffect(() => {
        const updateNearestPrayer = async () => {
            await invoke<any>('get_nearest_prayer')
                .then(result => setMyPrayerTimes(result))
                .catch(console.error);
        };

        updateNearestPrayer(); // Update immediately
        const interval = setInterval(updateNearestPrayer, 1000); // Update every second

        return () => clearInterval(interval); // Cleanup on unmount
    }, []);

    return myPrayerTimes;
};

export const useGetTimeUntilNearestPrayer = () => {
    const [myPrayerTimes, setMyPrayerTimes] = useState([] as any);

    useEffect(() => {
        const updateTimeNearestPrayer = async () => {
            await invoke<string>('get_time_until_next_prayer')
                .then(result => setMyPrayerTimes(result))
                .catch(console.error);
        };

        updateTimeNearestPrayer(); // Update immediately
        const interval = setInterval(updateTimeNearestPrayer, 1000); // Update every second

        return () => clearInterval(interval); // Cleanup on unmount
    }, []);

    return myPrayerTimes;
};