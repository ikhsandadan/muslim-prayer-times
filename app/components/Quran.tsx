"use client";
import { useState, useEffect } from 'react';
import CircularProgress from '@mui/material/CircularProgress';
import Divider from '@mui/material/Divider';
import CircleIcon from '@mui/icons-material/Circle';

import { useGetQuranData } from '../hook/useGetQuranData';
import Surah from './Surah';

const Quran = () => {
    const dataQuran = useGetQuranData();
    const [QuranData, setQuranData] = useState([] as any);
    const [content, setContent] = useState<string>("all");
    const [SurahData, setSurahData] = useState([] as any);
    const [id , setId] = useState<string>("");

    useEffect(() => {
        setQuranData(dataQuran.surahs);
    },[dataQuran]);

    const handleClickContent = (event: any, index: number, surah: any) => {
        event.preventDefault();
        setId(index.toString());
        setSurahData(surah);
        setContent("surah");
    };

    const handleContent = () => {
        if (content === "all") {
            return (
                <div className='flex flex-col mt-0 gap-2 mb-16 px-4 py-0'>
                    <h1 className="text-2xl font-bold">Quran</h1>
                    <Divider orientation="horizontal" variant="fullWidth" className='bg-white mb-4'/>
                    {QuranData?.length > 0 ? (
                        QuranData?.map((surah: any, index: number) => (
                            <div
                                key={index} 
                                className={`flex flex-row gap-2 ${index % 2 === 0 ? "bg-slate-900" : "bg-slate-800"} rounded py-6 px-10 justify-between cursor-pointer`}
                                onClick={(event)=> {handleClickContent(event, index + 1, surah) }}
                            >
                                <div className='relative flex-shrink-0 place-self-center'>
                                    <CircleIcon className='relative h-10 w-10' />
                                    <p className='absolute inset-0 flex items-center justify-center text-black font-bold'>{index + 1}</p>
                                </div>
                                <div className='flex flex-col flex-grow pl-4'>
                                    <p className='text-md'>{surah.english_name}</p>
                                    <p className='text-sm font-light text-gray-400'>{surah.english_name_translation}</p>
                                </div>
                                <h2 className='text-4xl self-center p-2'>{surah.name}</h2>
                            </div>
                        ))
                    ) : (
                        <div className='flex justify-center'>
                            <CircularProgress className='mt-8' />
                        </div>
                    )}
                </div>
            );
        } else if (content === "surah") {
            return (
                <Surah id={id} surah={SurahData} setContent={setContent} />
            );
        }
    };

    return (
        <>
            {handleContent()}
        </>
    );
};

export default Quran;