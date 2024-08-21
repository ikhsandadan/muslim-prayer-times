"use client";
import { useState } from 'react';
import Box from '@mui/material/Box';
import BottomNavigation from '@mui/material/BottomNavigation';
import BottomNavigationAction from '@mui/material/BottomNavigationAction';
import HomeIcon from '@mui/icons-material/Home';
import CalendarMonthIcon from '@mui/icons-material/CalendarMonth';
import MenuBookIcon from '@mui/icons-material/MenuBook';
import BarChartIcon from '@mui/icons-material/BarChart';
import Paper from '@mui/material/Paper';
import List from '@mui/material/List';
import { createTheme, ThemeProvider } from '@mui/material/styles';

import Calendar from './Calendar';
import HomePage from './HomePage';
import Quran from './Quran';
import Statistics from './Statistics';

const theme = createTheme({
    palette: {
        background: {
            default: '#000000',
        },
        primary: {
            main: '#00a360',
        },
        text: {
            primary: '#FFFFFF',
        },
    },
    components: {
        MuiBottomNavigationAction: {
            styleOverrides: {
                root: {
                    color: '#FFFFFF',
                    '&.Mui-selected': {
                        color: '#00a360',
                    },
                },
            },
        },
    },
});

export default function UseBottomNavigation() {
    const [value, setValue] = useState(0);

    const content = () => {
        if (value === 1) {
            return <Quran />;
        } else if (value === 2) {
            return <Calendar />;
        } else if (value === 3) {
            return <Statistics />;
        } else {
            return <HomePage />;
        }
    };

    return (
        <ThemeProvider theme={theme}>
        <Box sx={{ width: "100%", marginTop: 2 }}>
            <List className='justify-self-center'>
                {content()}
            </List>
            <Paper elevation={3} sx={{ position: 'fixed', bottom: 0, left: 0, right: 0 }}>
            <BottomNavigation
                showLabels
                value={value}
                onChange={(event, newValue) => {
                    setValue(newValue);
                }}
                sx={{ backgroundColor: 'black' }}
            >
                <BottomNavigationAction label="Home" icon={<HomeIcon />} />
                <BottomNavigationAction label="Quran" icon={<MenuBookIcon />} />
                <BottomNavigationAction label="Calendar" icon={<CalendarMonthIcon />} />
                <BottomNavigationAction label="Statistics" icon={<BarChartIcon />} />
            </BottomNavigation>
            </Paper>
        </Box>
        </ThemeProvider>
    );
};