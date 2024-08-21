'use client';
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';


export const useGetLocalTimes = () => {
    const [myTimes, setMyTimes] = useState('');

    useEffect(() => {
        const updateLocalTimes = async () => {
            await invoke<string>('get_local_time')
                .then(result => setMyTimes(result))
                .catch(console.error);
        };

        updateLocalTimes();
    }, []);

    return myTimes;
};

export const useGetLocalClock = () => {
    const [myTime, setMyTime] = useState('');

    useEffect(() => {
        const updateLocalTime = async () => {
            await invoke<string>('local_clock')
                .then(result => setMyTime(result))
                .catch(console.error);
        };

        updateLocalTime(); // Update immediately
        const interval = setInterval(updateLocalTime, 1000); // Update every second

        return () => clearInterval(interval); // Cleanup on unmount
    }, []);

    return myTime;
};

export const useGetLocalDate = () => {
    const [myDate, setMyDate] = useState('');

    useEffect(() => {
        const updateLocalDate = async () => {
            await invoke<string>('local_date')
                .then(result => setMyDate(result))
                .catch(console.error);
        };

        updateLocalDate();
    }, []);

    return myDate;
};

export const useGetLocalFormattedDate = () => {
    const [myDate, setMyDate] = useState('');

    useEffect(() => {
        const updateLocalDate = async () => {
            await invoke<string>('formatted_date')
                .then(result => setMyDate(result))
                .catch(console.error);
        };

        updateLocalDate();
    }, []);

    return myDate;
};

export const useGetTodayHijriDate = () => {
    const [myDate, setMyDate] = useState([] as any);

    useEffect(() => {
        const updateHijriDate = async () => {
            await invoke<string>('get_today_hijri_date')
                .then(result => setMyDate(result))
                .catch(console.error);
        };

        updateHijriDate();
    }, []);

    return myDate;
};

export const useGetCurrentMonthHijriCalendar = () => {
    const [hijriMonth, setHijriMonth] = useState([] as any);

    useEffect(() => {
        const updateHijriDate = async () => {
            await invoke<string>('get_hijri_calendar')
                .then(result => setHijriMonth(result))
                .catch(console.error);
        };

        updateHijriDate();
    }, []);

    return hijriMonth;
};