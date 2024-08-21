"use client";
import { FC, useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import dayjs, { Dayjs } from 'dayjs';
import { LocalizationProvider } from '@mui/x-date-pickers/LocalizationProvider';
import { DateCalendar } from '@mui/x-date-pickers/DateCalendar';
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs';
import { IconButton } from '@mui/material';
import { DayCalendarSkeleton } from '@mui/x-date-pickers/DayCalendarSkeleton';
import { PickersDay, PickersDayProps } from '@mui/x-date-pickers/PickersDay';
import Badge from '@mui/material/Badge';
import Divider from '@mui/material/Divider';
import CheckCircleOutlineIcon from '@mui/icons-material/CheckCircleOutline';
import RadioButtonUncheckedIcon from '@mui/icons-material/RadioButtonUnchecked';

import { useGetLocalTimes, useGetLocalDate, useGetTodayHijriDate } from '../hook/useGetLocalTimes';


interface PrayerRecordRowProps {
    label: string;
    isChecked: boolean;
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

function ServerDay(props: PickersDayProps<Dayjs> & { holidayDate?: number[] }) {
    const { holidayDate = [], day, outsideCurrentMonth, ...other } = props;
    
    const isSelected = !props.outsideCurrentMonth && holidayDate.indexOf(props.day.date()) >= 0;
    
    return (
        <Badge
            key={props.day.toString()}
            overlap="circular"
            badgeContent={isSelected ? '🟢' : undefined}
        >
            <PickersDay {...other} outsideCurrentMonth={outsideCurrentMonth} day={day} />
        </Badge>
    );
};

const Calendar = () => {
    const todayDate = useGetLocalDate();
    const localDate = useGetLocalTimes();
    const thisMonth = todayDate.split('-')[1];
    const thisYear = todayDate.split('-')[2];
    const todayHijriDate = useGetTodayHijriDate();
    const [value, setValue] = useState<Dayjs | null>(null);
    const [isCurrentMonth, setIsCurrentMonth] = useState(true);
    const [hijriHolidayData, setHijriHolidayData] = useState([]);
    const [holidayDate, setHolidayDate] = useState();
    const [isLoading, setIsLoading] = useState(false);
    const [prayerRecords, setPrayerRecords] = useState<PrayerRecord[]>([]);

    const handleFormatDate = (dateStr: string): string => {
        const [day, month, year] = dateStr.split('-');
    
        const date = new Date(`${year}-${month}-${day}`);
    
        const options: Intl.DateTimeFormatOptions = { year: 'numeric', month: 'long', day: 'numeric' };
        const formattedDate = date.toLocaleDateString('en-GB', options);
    
        return formattedDate;
    };

    const handleFormateDatePrayerRecords = (dateStr: string): string => {
        const date = new Date(dateStr);
        const options: Intl.DateTimeFormatOptions = { year: 'numeric', month: 'long', day: 'numeric' };
        const formattedDate = date.toLocaleDateString('en-GB', options);
    
        return formattedDate;
    };

    const fetchPrayersRecord = async (date: string) => {
        try {
            const result: PrayerRecord[] = await invoke('get_prayer_data_by_date', { date });
            return result;
        } catch (error: any) {
            console.error(error);
            return [];
        }
    };

    const checkHijriHoliday = async (month: String, year: String) => {
        await invoke<any>('check_holidays', { month: month, year: year })
            .then(result => setHijriHolidayData(result))
            .catch(console.error);

        await invoke<any>('get_holiday_days', { month: month, year: year })
            .then(result => setHolidayDate(result))
            .catch(console.error);
    };

    useEffect(() => {
        if (localDate) {
            setValue(dayjs(localDate.split('T')[0]));
        }
    }, [localDate]);

    useEffect(() => {
        if (thisMonth && thisYear) {
            checkHijriHoliday(thisMonth, thisYear);
        }
    }, [thisMonth, thisYear]);

    useEffect(() => {
        const fetchData = async () => {
            if (value) {
                const prayerRecord = await fetchPrayersRecord(value.format('YYYY-MM-DD'));
                setPrayerRecords(prayerRecord);
            }
        }

        fetchData();
    }, [value]);

    const handleMonthChange = async (date: Dayjs) => {
        const newMonth = date.format('MM');
        const newYear = date.format('YYYY');
        const currentDate = dayjs();

        setIsLoading(true);
        await checkHijriHoliday(newMonth, newYear);
        setIsCurrentMonth(
            date.month() === currentDate.month() && 
            date.year() === currentDate.year()
        );
        setIsLoading(false);
    };

    const handleTodayButton = () => {
        setValue(dayjs(localDate.split('T')[0]));
        setIsCurrentMonth(true);
    };

    const PrayerRecordRow: FC<PrayerRecordRowProps> = ({ label, isChecked }) => (
        <div className='flex flex-row justify-between items-center content-center px-4 py-2'>
            <div className='text-sm font-bold'>{label}</div>
            {isChecked ? <CheckCircleOutlineIcon /> : <RadioButtonUncheckedIcon />}
        </div>
    );

    return (
        <div className='flex flex-col mt-0 gap-2 mb-16 px-4 py-0 overflow-visible'>
            <h1 className="text-2xl font-bold self-start">Calendar</h1>
            <Divider orientation="horizontal" variant="fullWidth" className="bg-white mb-2"/>
            <div className='flex flex-col gap-0'>
                <h2 className='text-lg text-center'>{todayHijriDate.day} {todayHijriDate.month} {todayHijriDate.year}</h2>
                <h3 className='text-sm font-extralight text-center'>{handleFormatDate(todayDate)}</h3>
            </div>
            <div className="w-full h-full flex justify-center overflow-visible">
            <LocalizationProvider dateAdapter={AdapterDayjs}>
                <DateCalendar 
                    views={['day']} 
                    value={value}
                    loading={isLoading}
                    renderLoading={() => <DayCalendarSkeleton />}
                    onChange={(newValue) => setValue(newValue)}
                    onMonthChange={handleMonthChange}
                    slots={{
                        previousIconButton: IconButton,
                        nextIconButton: IconButton,
                        day: ServerDay,
                    }}
                    slotProps={{
                        previousIconButton: { 
                            sx: { color: 'white' } 
                        },
                        nextIconButton: { 
                            sx: { color: 'white' } 
                        },
                        day: {
                            holidayDate,
                        } as any,
                    }}
                    sx={{
                        width: '80%',
                        height: 5000,
                        '& .MuiPickersCalendarHeader-root': {
                            display: 'flex',
                            justifyContent: 'space-between',
                            alignItems: 'center',
                            marginBottom: '2.5rem',
                            marginTop: '0rem',
                        },
                        '& .MuiPickersSlideTransition-root': {
                            overflow: 'visible',
                        },
                        '& .MuiPickersCalendarHeader-label': {
                            color: 'white',
                            fontSize: '1.25rem',
                        },
                        '& .MuiTypography-root': {
                            fontSize: '1.25rem',
                            color: 'white',
                        },
                        '& .MuiDayCalendar-weekDayLabel': {
                            color: 'white',
                            fontSize: '1.25rem',
                            height: 'auto',
                            lineHeight: 'normal',
                        },
                        '& .MuiPickersDay-root': {
                            fontSize: '1.25rem',
                            margin: '6px',
                            padding: '4px',
                            height: 'auto',
                            lineHeight: 'normal',
                        },
                        '& .MuiDayCalendar-header': {
                            display: 'flex',
                            justifyContent: 'space-between',
                            alignItems: 'center',
                            '& .MuiTypography-root': {
                                fontSize: '1.25rem',
                            },
                            marginBottom: '2rem',
                        },
                        '& .MuiDayCalendar-weekContainer': {
                            display: 'flex',
                            justifyContent: 'space-between',
                            alignItems: 'center',
                        },
                        '& .MuiDateCalendar-root': {
                            height: '100%',
                            display: 'flex',
                            flexDirection: 'column',
                            justifyContent: 'space-between',
                        }
                    }}
                    className='flex overflow-visible'
                />
            </LocalizationProvider>
        </div>
            {(!isLoading && (!isCurrentMonth || value?.format('YYYY-MM-DD') !== dayjs().format('YYYY-MM-DD'))) ? (
                <div className='mt-16 self-center'>
                    <button 
                        onClick={handleTodayButton} 
                        className='bg-slate-700 text-sm text-white px-4 py-2 rounded hover:bg-[#00a360] transition duration-300 ease-in-out'
                    >
                        Today
                    </button>
                </div>
            ) : null}
            <div className={`${value?.format('YYYY-MM-DD') !== dayjs().format('YYYY-MM-DD') ? 'mt-2' : 'mt-16'} flex ${hijriHolidayData.length > 0 ? 'justify-evenly' : 'justify-center'}`}>
                {hijriHolidayData.length > 0 && !isLoading ? (
                    <div className='bg-slate-900 rounded-lg w-80'>
                        {hijriHolidayData?.map((holiday: any, index: number) => (
                            <div key={index} className={`flex flex-col ${index > 0 ? '' : 'mt-2'}`}>
                                <div className='flex flex-col px-4 pb-2'>
                                    <div className='flex flex-row gap-2 text-md'>{holiday?.holidays.map((h: any, i: number) => <p key={i}>{h}{holiday?.holidays?.length > 0 && i < holiday?.holidays?.length - 1 ? ', ' : ''}</p>)}</div>
                                    <div className='text-sm font-semilight text-gray-400'>{holiday?.day} {holiday?.month} {holiday?.year}</div>
                                    <div className='text-sm font-semilight text-gray-400'>{handleFormatDate(holiday?.gregorian_date)}</div>
                                </div>
                                {hijriHolidayData.length > 1 && index < hijriHolidayData.length - 1 ? <Divider orientation="horizontal" variant="fullWidth" className="bg-white mb-2" /> : null}
                            </div>
                        ))}
                    </div>
                ):(null)}
                {!isLoading ? (
                    <div className='bg-slate-900 rounded-lg w-80'>
                        <h1 className='text-lg font-bold px-4 py-2 text-center'>Prayer Record Each Day</h1>
                        {prayerRecords.length > 0 ? (
                            prayerRecords?.map((record: any, index: number) => (
                                <div key={index} className='flex flex-col'>
                                    <div className='flex flex-col pb-2'>
                                        <div className='text-sm font-bold text-center mb-2'>{handleFormateDatePrayerRecords(record?.date)}</div>
                                        <div className='flex flex-col justify-between gap-2'>
                                            <PrayerRecordRow label="Fajr" isChecked={record.fajr} />
                                            <Divider orientation="horizontal" variant="fullWidth" className="bg-black" style={{ height: '2px' }} />
                                            <PrayerRecordRow label="Dhuhr" isChecked={record.dhuhr} />
                                            <Divider orientation="horizontal" variant="fullWidth" className="bg-black" style={{ height: '2px' }} />
                                            <PrayerRecordRow label="Asr" isChecked={record.asr} />
                                            <Divider orientation="horizontal" variant="fullWidth" className="bg-black" style={{ height: '2px' }} />
                                            <PrayerRecordRow label="Maghrib" isChecked={record.maghrib} />
                                            <Divider orientation="horizontal" variant="fullWidth" className="bg-black" style={{ height: '2px' }} />
                                            <PrayerRecordRow label="Isha" isChecked={record.isha} />
                                        </div>
                                    </div>
                                </div>
                            ))
                        ) : (<div className='text-center mb-4'>No prayer record found. Please select a date.</div>)}
                    </div>
                ):(null)}
            </div>
        </div>
    );
};

export default Calendar;
