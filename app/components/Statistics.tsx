"use client";
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import Divider from '@mui/material/Divider';
import { Dayjs } from 'dayjs';
import { DateRangePicker } from 'react-date-range';
import { addDays } from 'date-fns';

import 'react-date-range/dist/styles.css';
import 'react-date-range/dist/theme/default.css';
import './customDateRangeStyles.css';

const Statistics = () => {
    const [svg, setSvg] = useState();
    const [state, setState] = useState([
        {
            startDate: new Date(),
            endDate: addDays(new Date(), 7),
            key: 'selection'
        }
    ]);

    const generateHeatmapByRange = async () => {
        const formatDate = (date: Date | null) => {
            return date ? date.toLocaleDateString('en-CA') : '';
        };

        try {
            await invoke<any>('get_prayer_heatmap_by_range', { userId: 1, startDate: formatDate(state[0].startDate), endDate: formatDate(state[0].endDate) })
                .then((res) => { setSvg(res); })
                .catch((err) => { console.error(err); });
        } catch (error) {
        console.error('Error generating heatmap:', error);
        }
    };

    useEffect(() => {
        generateHeatmapByRange()
    }, [state]);

    return (
        <div className='flex flex-col mt-0 gap-2 mb-16 px-4 py-0'>
            <h1 className="text-2xl font-bold">Statistics</h1>
            <Divider orientation="horizontal" variant="fullWidth" className='bg-white mb-4'/>
            <div className='flex flex-col items-center self-center min-w-[300px]'>
                <DateRangePicker
                    onChange={(item: any) => setState([item.selection])}
                    moveRangeOnFirstSelection={false}
                    months={2}
                    ranges={state}
                    direction="horizontal"
                    className="mt-2 mx-2 custom-date-range-picker"
                />
            </div>
            <div className='flex items-center self-center mt-2'>
                {svg ? (
                    <div dangerouslySetInnerHTML={{ __html: svg }} />
                ) : (null)}
            </div>
        </div>
    )
};

export default Statistics;