use curl::easy::Easy;

// Requests should shoot off into their own thread, so we can keep the UI responsive.
//