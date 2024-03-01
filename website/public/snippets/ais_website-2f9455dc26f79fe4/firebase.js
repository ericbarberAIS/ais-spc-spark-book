// firebase.js

// CDN imports
import { initializeApp } from 'https://www.gstatic.com/firebasejs/10.8.0/firebase-app.js';
import { getAnalytics } from "https://www.gstatic.com/firebasejs/10.8.0/firebase-analytics.js";
import { getFunctions, httpsCallable } from 'https://www.gstatic.com/firebasejs/10.8.0/firebase-functions.js';

// Import the functions you need from the SDKs you need
// import { initializeApp } from "firebase/app";
// import { getAnalytics } from "firebase/analytics";
// import { getFunctions, httpsCallable } from 'firebase/functions';
// TODO: Add SDKs for Firebase products that you want to use
// https://firebase.google.com/docs/web/setup#available-libraries

// Your web app's Firebase configuration
// For Firebase JS SDK v7.20.0 and later, measurementId is optional
const firebaseConfig = {
  apiKey: "AIzaSyAzb1sluAwS2LwR-JYTh_lxOSYaBsVzKuQ",
  authDomain: "ais-website-b0380.firebaseapp.com",
  projectId: "ais-website-b0380",
  storageBucket: "ais-website-b0380.appspot.com",
  messagingSenderId: "702381011716",
  appId: "1:702381011716:web:9939bfa939604eaf76d651",
  measurementId: "G-SWV4G6BPKN"
};

// Initialize Firebase
const app = initializeApp(firebaseConfig);
const analytics = getAnalytics(app);
const functions = getFunctions(app);

export const sendFirebaseEmail = (emailData) => {
  const sendEmailFunction = httpsCallable(functions, 'sendContactEmail');
  return sendEmailFunction(emailData);
};

export default app;
