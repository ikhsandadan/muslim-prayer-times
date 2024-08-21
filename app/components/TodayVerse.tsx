"use client";
import Skeleton from '@mui/material/Skeleton';
import { useGetTodayVerse } from '../hook/useGetTodayVerse';

const TodayVerse = () => {
    const todayVerse = useGetTodayVerse();

    return (
        <>
        {todayVerse.length !== 0 ? (
            <div className='flex flex-col gap-2 justify-between p-2 mx-4 mt-6 bg-slate-700 rounded-lg'>
                <h1 className='text-2xl font-bold'>Daily Verse</h1>
                <p className='text-sm font-light'>{todayVerse.surah_name}({todayVerse?.surah_name_translation}) ({todayVerse?.surah_number}:{todayVerse?.verse_number})</p>
                <p className='text-md'>{todayVerse.verse_text}</p>
            </div>
        ) : (
            <div className='flex flex-col gap-4 justify-between h-48 p-4 relative'>
                <Skeleton variant="rounded" width='100%' height='100%' />
            </div>
        )}
        </>
    );
};

export default TodayVerse;