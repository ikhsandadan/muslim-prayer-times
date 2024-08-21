"use client";
import { useState, useEffect } from 'react';
import Skeleton from '@mui/material/Skeleton';

import { useGetLocalFormattedDate, useGetTodayHijriDate } from '../hook/useGetLocalTimes';
import { useGetNearestPrayer, useGetTimeUntilNearestPrayer } from '../hook/useGetPrayerTimes';
import { useGetLocation } from '../hook/useGetLocation';
import { image } from '../utils/image';
import TodayVerse from './TodayVerse';
import AddPrayer from './AddPrayer';

const HomePage = () => {
    const myLocation = useGetLocation();
    const localFormattedDate = useGetLocalFormattedDate();
    const todayHijriDate = useGetTodayHijriDate();
    const nearestPrayer = useGetNearestPrayer();
    const timeUntilNextPrayer = useGetTimeUntilNearestPrayer();
    const [bgImage, setBgImage] = useState<string>('');

    useEffect(() => {
        if (nearestPrayer === 'Fajr') {
            setBgImage(image.dawn.src);
        } else if (nearestPrayer === 'Maghrib') {
            setBgImage(image.dusk.src);
        } else if (nearestPrayer === 'Isha') {
            setBgImage(image.night.src);
        } else {
            setBgImage(image.day.src);
        }
    }, [nearestPrayer]);

    return (
        <div className='flex flex-col mt-0 gap-2 mb-16'>
            <div className='flex flex-col'>
                    <h2 className='text-center'>Today, {localFormattedDate}</h2>
                    <h3 className='text-sm text-center'>{todayHijriDate.day} {todayHijriDate.month} {todayHijriDate.year}</h3>
            </div>
            {nearestPrayer.length > 0 && timeUntilNextPrayer ? (
                <div className='flex flex-col gap-4 justify-between h-72 p-4 relative'>
                    <div className='absolute top-10 left-8 right-0 bottom-0 flex flex-row justify-between'>
                        <div className='flex flex-col'>
                            <h1 className='text-4xl font-bold drop-shadow backdrop-blur-sm bg-[#00a360]/40 rounded-lg px-4 py-2'>{nearestPrayer}</h1>
                        </div>
                        <div className='mr-8 mt-2'>
                            <h1 className='text-md font-bold drop-shadow backdrop-blur-sm bg-[#00a360]/40 rounded-lg px-2 py-0'>{timeUntilNextPrayer}</h1>
                        </div>
                    </div>

                    <div className='absolute top-70 left-8 right-0 bottom-0 flex'>
                        <h2 className='text-xl font-bold drop-shadow backdrop-blur-sm bg-[#00a360]/40 rounded-lg px-2 py-0'>{myLocation?.city?.replace(/^"|"$/g, '')}</h2>
                    </div>

                    <img src={bgImage} alt="Background" className='w-full h-72 object-cover rounded-lg' />
                </div>
            ) : (
                <div className='flex flex-col gap-4 justify-between h-80 p-4 relative'>
                    <Skeleton variant="rounded" width='100%' height='100%' />
                </div>
            )}

            <TodayVerse />
            <AddPrayer />
        </div>
    )
}

export default HomePage;