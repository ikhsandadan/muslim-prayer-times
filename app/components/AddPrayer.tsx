import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import Skeleton from '@mui/material/Skeleton';

import { useGetLocalTimes } from '../hook/useGetLocalTimes';
import { useGetNearestPrayer } from '../hook/useGetPrayerTimes';

interface Prayers {
    fajr: boolean;
    dhuhr: boolean;
    asr: boolean;
    maghrib: boolean;
    isha: boolean;
};

interface PrayerRecord {
    user_id: number;
    date: string;
    fajr: boolean;
    dhuhr: boolean;
    asr: boolean;
    maghrib: boolean;
    isha: boolean;
};

export default function AddPrayer() {
    const nearestPrayer = useGetNearestPrayer();
    const localTimes = useGetLocalTimes();
    const [date, setDate] = useState<string>('');
    const [prayers, setPrayers] = useState<Prayers>({ 
        fajr: false,
        dhuhr: false,
        asr: false,
        maghrib: false,
        isha: false,
    });

    const fetchPrayersRecord = async (date: string) => {
        try {
            const result: PrayerRecord[] = await invoke('get_prayer_data_by_date', { date: date });
            return result;
        } catch (error: any) {
            console.error(error);
            return [];
        }
    };

    useEffect(() => {
        setDate(localTimes.split('T')[0]);
    }, [localTimes]);

    useEffect(() => {
        const fetchRecords = async () => {
            try {
                const result: PrayerRecord[] = await fetchPrayersRecord(date);

                if (result.length > 0) {
                const latestRecord = result[0];
                setPrayers({
                    fajr: latestRecord.fajr,
                    dhuhr: latestRecord.dhuhr,
                    asr: latestRecord.asr,
                    maghrib: latestRecord.maghrib,
                    isha: latestRecord.isha,
                });
                }
            } catch (error: any) {
                console.error(error);
            }
        };

        fetchRecords();
    }, [nearestPrayer]);

    const handleToggle = (prayer: keyof Prayers) => {
        setPrayers((prev) => ({ ...prev, [prayer]: !prev[prayer] }));
    };
    
    const handleSubmit = async () => {
        try {
            await invoke('add_prayer', {
                userId: 1,
                date: date,
                ...prayers,
            });

            const result: PrayerRecord[] = await fetchPrayersRecord(date);
            
            alert('Prayer record added successfully!');
        } catch (error: any) {
            const result: PrayerRecord[] = await fetchPrayersRecord(date);

            console.error(error);
            alert('Failed to add prayer record.');
        }
    };

    return (
        <>
        {nearestPrayer.length > 0 ? (
            <div className="bg-green-900 flex flex-col gap-2 p-2 mx-4 mt-2 items-center justify-center rounded-lg">
                <h1 className="text-xl font-bold text-white mb-8 self-start">Keep Going!</h1>
                <div className="flex space-x-4">
                    {(['fajr', 'dhuhr', 'asr', 'maghrib', 'isha'] as (keyof Prayers)[]).map((prayer, index) => (
                    <button
                        key={index}
                        onClick={() => handleToggle(prayer)}
                        className={`w-20 h-20 rounded-full ${
                        prayers[prayer] ? 'bg-[#00a360] border-4 border-white' : 'bg-slate-600 text-white hover:border-white'
                        } flex items-center justify-center hover:border-4`}
                    >
                        {prayer.charAt(0).toUpperCase() + prayer.slice(1)}
                    </button>
                    ))}
                </div>
                <button
                    onClick={handleSubmit}
                    className="mt-6 mb-2 px-4 py-2 bg-slate-600 text-white rounded hover:bg-[#00a360] hover:border-2 hover:border-white transition duration-300 ease-in-out"
                >
                    Submit
                </button>
            </div>
        ) : (
            <div className='flex flex-col gap-4 justify-between h-48 p-4 relative'>
                <Skeleton variant="rounded" width='100%' height='100%' />
            </div>
        )}
        </>
    );
};