const puppeteer = require('puppeteer-core');

(async () => {

  // To run by default (in headless mode)  
  // const browser = await puppeteer.launch();
  // const browser = await puppeteer.launch({});
  
  const profile = 'Profile 3'
  const profilePath = '/Users/sagarmhatre/Library/Application Support/Google/Chrome';


//   
  const browser = await puppeteer.launch({
    headless: false,
    executablePath: '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',
    args:[
        '--user-data-dir=' + profilePath,
        '--profile-directory=' + profile
        //,'--incognito'
    ]
    });
  const page = await browser.newPage();
  await page.goto('https://example.com');
  await page.screenshot({ path: 'example.png' });

  // Go to Gmail login page
  await page.goto('https://accounts.google.com/');

  // Enter username and password
//   await page.type('input[type="email"]', 'hirenaryan2016');
//   await page.click('div[id="identifierNext"]');

//   await page.waitForSelector('input[type="password"]');
//   await page.type('input[type="password"]', 'your_password');
  // await page.click('div[id="passwordNext"]');

  // Wait for login to complete
  await page.waitForNavigation();

  //await browser.close();
})();

// node 001_take_screenshot.js