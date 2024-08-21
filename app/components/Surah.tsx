"use client";
import { Dispatch, FC, useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import AppBar from '@mui/material/AppBar';
import Toolbar from '@mui/material/Toolbar';
import ArrowBackIcon from '@mui/icons-material/ArrowBack';
import PlayCircleIcon from '@mui/icons-material/PlayCircle';
import StopCircleIcon from '@mui/icons-material/StopCircle';
import Slider from '@mui/material/Slider';
import VolumeDown from '@mui/icons-material/VolumeDown';
import VolumeUp from '@mui/icons-material/VolumeUp';
import LinearProgress, { LinearProgressProps } from '@mui/material/LinearProgress';

import { useGetSurahTranslation } from '../hook/useGetSurahTranslation';

interface SurahProps {
    id: string;
    surah: any;
    setContent: Dispatch<string>;
};

const Surah: FC<SurahProps> = ({ id, surah, setContent }) => {
    const dataSurah = useGetSurahTranslation({ id });
    const [surahTranslation, setSurahTranslation] = useState([] as any);
    const [currentAyah, setCurrentAyah] = useState<string | null>("");
    const [currentAyahNumber, setCurrentAyahNumber] = useState<number>(0);
    const [currentAyahUrl, setCurrentAyahUrl] = useState<string | null>("");
    const [isPlaying, setIsPlaying] = useState<boolean>(false);
    const [isPaused, setIsPaused] = useState<boolean>(false);
    const [volume, setVolume] = useState(100);
    const [progress, setProgress] = useState(0);

    const handleAudioFinished = () => {
        setProgress(100);
        setIsPlaying(false);
    };

    const playAudioWithProgress = async (ayah: string, ayahNumber: number, url: string) => {
        setCurrentAyah(ayah);
        setCurrentAyahNumber(ayahNumber);
        setCurrentAyahUrl(url);
        setIsPlaying(true);
        setIsPaused(false);
        setProgress(0);

        const normalizedVolume = volume / 100;
    
        await invoke('play_audio', { url: url, vol: normalizedVolume });
    };

    const playAudio = (ayah: string, ayahNumber: number, url: string) => {
        playAudioWithProgress(ayah, ayahNumber, url);
    };

    const pauseAudio = async () => {
        await invoke('pause_audio');
        setProgress(0);
        setIsPaused(true);
        setIsPlaying(false);
    };

    const changeVolume = async (event: Event, newValue: number | number[]) => {
        event.preventDefault();
        const newVolume = newValue as number;
        const normalizedVolume = newVolume / 100;
        setVolume(newVolume);

        await invoke('set_volume', { vol: normalizedVolume });
    };

    useEffect(() => {
        setSurahTranslation(dataSurah);
    }, [dataSurah]);

    useEffect(() => {
        const setupProgressListener = async () => {
            const unlistenProgress = await listen('audio-progress', (event: any) => {
                if (isPlaying && !isPaused) {
                    if (event.payload >= 1) {
                        setProgress(100);
                        setIsPlaying(false);
                    } else {
                        setProgress(event.payload * 100);
                    }
                }
            });
        
            return unlistenProgress;
        };
    
        const setupFinishedListener = async () => {
            const unlistenFinished = await listen('audio-finished', () => {
                handleAudioFinished();
            });
        
            return unlistenFinished;
        };
    
        // Setup listeners
        const setupListeners = async () => {
            const unlistenProgress = await setupProgressListener();
            const unlistenFinished = await setupFinishedListener();
        
            // Cleanup listeners
            return () => {
                unlistenProgress();
                unlistenFinished();
            };
        };
    
        const cleanupListeners = setupListeners();
    
        return () => {
            cleanupListeners.then((cleanup) => cleanup());
        };
    }, [isPlaying, isPaused]);

    // Reset progress when currentAyah changes
    useEffect(() => {
        if (currentAyah) {
            setProgress(0);
        }
    }, [currentAyah]);

    const handleAppBarPlayClick = () => {
        if (!isPlaying) {
            if (!currentAyah || !currentAyahUrl) {
                const firstAyah = surah.ayahs[0];
                playAudioWithProgress(firstAyah.text, 1, firstAyah.audio);
            } else {
                playAudioWithProgress(currentAyah, currentAyahNumber, currentAyahUrl);
            }
        } else {
            pauseAudio();
        }
    };

    const LinearProgressWithLabel = (props: LinearProgressProps & { value: number }) => {
        return (
            <LinearProgress variant="determinate" {...props} sx={{ height: 6, borderRadius: 5, width: '80%', position: 'absolute', bottom: 57, left: 68 }} />
        );
    };

    return (
        <div className='flex flex-col mt-0 gap-2 mb-16 px-4 py-0'>
            <AppBar position="fixed" className='bg-black'>
                <Toolbar>
                    <div className='flex flex-row gap-10 my-4'>
                        <ArrowBackIcon className='self-center h-10 w-10 cursor-pointer rounded-full hover:bg-slate-600 transition ease-in-out duration-300' onClick={() => setContent("all")}/>
                        <div className='flex flex-col'>
                            <h1 className="text-2xl font-bold">{surah.english_name}</h1>
                            <p className='text-md font-light text-gray-400'>{surah.english_name_translation}</p>
                        </div>
                    </div>
                </Toolbar>
            </AppBar>

            <div className='flex flex-col gap-4'>
                {surah?.ayahs?.map((verse: any, index: number) => (
                    <div key={index}  className={`flex flex-col  px-4 py-8 rounded-lg  ${index === 0 ? "text-center mt-20" : "text-end"} ${index % 2 === 0 ? "bg-slate-900" : "bg-slate-800"} ${index === surah?.ayahs.length - 1 ? "mb-16" : ""}`}>
                        <div className={`text-4xl font-light `}>
                            {index + 1}. {verse.text}
                        </div>
                        <div className={`flex gap-2 ${index === 0 && verse.text === "بِسْمِ ٱللَّهِ ٱلرَّحْمَٰنِ ٱلرَّحِيمِ" ? "text-center" : "text-start"} mt-8`}>
                            <p>{index + 1}.</p>
                            <p>{surahTranslation?.[index]?.text}</p>
                        </div>
                        <div className='flex justify-end'>
                            {isPlaying && currentAyah === verse.text ? (
                                <StopCircleIcon className='h-10 w-10 cursor-pointer' onClick={pauseAudio}/>
                            ) : (
                                <PlayCircleIcon className='h-10 w-10 cursor-pointer' onClick={() => playAudio(verse.text, index + 1, verse.audio)}/>
                            )}
                        </div>
                    </div>
                ))}
            </div>

            {isPlaying || currentAyah ? (
                <AppBar position="fixed" className='bg-black' style={{ top: 'auto', bottom: 56 }}>
                    <Toolbar className='flex flex-row w-full justify-between items-center relative'>
                        {isPlaying ? (
                            <LinearProgressWithLabel value={progress} />
                        ) : (null)}
                        {currentAyahNumber !== 0 ? (
                            <div className='absolute left-7 font-bold'>{surah?.english_name}-{currentAyahNumber}</div>
                        ) : (null)}
                        <div className='absolute left-1/2 transform -translate-x-1/2'>
                            {isPlaying ? (
                                <StopCircleIcon className='h-10 w-10 cursor-pointer' onClick={pauseAudio}/>
                            ) : (
                                <PlayCircleIcon className='h-10 w-10 cursor-pointer' onClick={handleAppBarPlayClick}/>
                            )}
                        </div>

                        <div className='flex flex-row items-center ml-auto gap-2'>
                            <VolumeDown />
                            <Slider aria-label="Volume" value={volume} onChange={changeVolume} className='w-32 mx-2'/>
                            <VolumeUp className='ml-2'/>
                        </div>
                    </Toolbar>
                </AppBar>
            ) : (null)}
        </div>
    )
};

export default Surah;