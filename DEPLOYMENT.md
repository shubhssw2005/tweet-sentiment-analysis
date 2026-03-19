# Deployment Guide

This guide covers deploying the Tweet Sentiment Analysis application to production.

## Prerequisites

- GitHub account
- RapidAPI key for Twitter241 API
- Render account (for backend)
- Netlify account (for frontend)

## Step 1: Push to GitHub

```bash
# Initialize git (if not already done)
git init

# Add all files
git add .

# Commit
git commit -m "Initial commit: Tweet Sentiment Analysis"

# Add remote
git remote add origin https://github.com/yourusername/tweet-sentiment-analysis.git

# Push to GitHub
git branch -M main
git push -u origin main
```

## Step 2: Deploy Backend on Render

### Option A: Using Render Dashboard

1. Go to [render.com](https://render.com)
2. Sign up/Login with GitHub
3. Click "New +" → "Web Service"
4. Connect your GitHub repository
5. Configure:
   - **Name**: `tweet-sentiment-api`
   - **Runtime**: `Rust`
   - **Build Command**: `cargo build --release`
   - **Start Command**: `./target/release/sentiment-api`
   - **Plan**: Free (or paid for better performance)

6. Add Environment Variables:
   - `RAPIDAPI_KEY`: Your API key
   - `RUST_LOG`: `info`

7. Click "Create Web Service"
8. Wait for deployment (5-10 minutes)
9. Copy the URL (e.g., `https://tweet-sentiment-api.onrender.com`)

### Option B: Using Railway

1. Go to [railway.app](https://railway.app)
2. Click "New Project"
3. Select "Deploy from GitHub repo"
4. Connect your repository
5. Add environment variables
6. Railway auto-detects Rust and deploys

## Step 3: Deploy Frontend on Netlify

### Option A: Using Netlify Dashboard

1. Go to [netlify.com](https://netlify.com)
2. Sign up/Login with GitHub
3. Click "Add new site" → "Import an existing project"
4. Select your GitHub repository
5. Configure:
   - **Build command**: Leave empty (static site)
   - **Publish directory**: `.` (root)

6. Click "Deploy site"
7. Wait for deployment (1-2 minutes)
8. Copy the URL (e.g., `https://tweet-sentiment.netlify.app`)

### Option B: Drag and Drop

1. Go to [netlify.com](https://netlify.com)
2. Drag and drop `index.html` to deploy
3. Get instant URL

## Step 4: Update Frontend API URL

After backend deployment, update the frontend to use your backend URL:

### Edit `index.html`

Find this line (around line 430):

```javascript
const API_URL = "http://127.0.0.1:8080";
```

Replace with your Render URL:

```javascript
const API_URL = "https://tweet-sentiment-api.onrender.com";
```

Then redeploy to Netlify:

```bash
git add index.html
git commit -m "Update API URL for production"
git push origin main
```

Netlify will auto-redeploy.

## Step 5: Test Deployment

1. Open your Netlify URL in browser
2. Enter topics (e.g., "bitcoin, crypto")
3. Click "Start Stream"
4. Wait 10 seconds for tweets to appear
5. Verify sentiment analysis is working

## Troubleshooting

### Backend not starting

- Check logs on Render dashboard
- Verify `RAPIDAPI_KEY` is set correctly
- Ensure Python dependencies are installed

### Frontend not connecting to backend

- Check API URL in `index.html`
- Verify backend is running
- Check browser console for CORS errors
- Add CORS headers if needed

### No tweets appearing

- Verify API key is valid
- Check backend logs
- Ensure internet connection
- Try different topics

### Slow performance

- Upgrade to paid plan on Render
- Enable caching on Netlify
- Optimize database queries

## Continuous Deployment

The GitHub Actions workflow automatically:

- Builds and tests on every push
- Deploys to Render (backend)
- Deploys to Netlify (frontend)

To enable:

1. Add secrets to GitHub:
   - `RENDER_SERVICE_ID`
   - `RENDER_API_KEY`
   - `NETLIFY_AUTH_TOKEN`
   - `NETLIFY_SITE_ID`

2. Push to main branch
3. Automatic deployment starts

## Monitoring

### Render Dashboard

- View logs
- Monitor performance
- Check health status

### Netlify Dashboard

- View deployment history
- Check build logs
- Monitor analytics

## Scaling

### For more traffic:

1. Upgrade Render plan
2. Enable auto-scaling
3. Add caching layer
4. Optimize database

### For more tweets:

1. Increase `MAX_TWEETS` in code
2. Add database for persistence
3. Implement pagination

## Security

1. Never commit `.env` file
2. Use environment variables for secrets
3. Enable HTTPS (automatic on Render/Netlify)
4. Add rate limiting
5. Validate all inputs

## Rollback

If deployment fails:

```bash
# Revert last commit
git revert HEAD

# Push to trigger redeploy
git push origin main
```

## Support

For deployment issues:

- Check Render/Netlify documentation
- Review GitHub Actions logs
- Check application logs
- Open GitHub issue

## Next Steps

After deployment:

1. Share your live URL
2. Monitor performance
3. Gather user feedback
4. Plan improvements
5. Add more features
