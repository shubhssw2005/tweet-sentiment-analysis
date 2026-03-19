# GitHub & Deployment Setup - Step by Step

Follow these exact steps to deploy your project.

## Step 1: Initialize Git & Push to GitHub

```bash
cd /Users/shubhamsw2005/twitter-sentiment-2026

# Initialize git
git init

# Add all files
git add .

# Create initial commit
git commit -m "Initial commit: Tweet Sentiment Analysis System"

# Create main branch
git branch -M main

# Add remote (replace with your GitHub URL)
git remote add origin https://github.com/yourusername/tweet-sentiment-analysis.git

# Push to GitHub
git push -u origin main
```

## Step 2: Deploy Backend on Render (FREE)

### 2.1 Create Render Account

- Go to https://render.com
- Sign up with GitHub
- Authorize Render to access your repositories

### 2.2 Create Web Service

1. Click "New +" button
2. Select "Web Service"
3. Select your GitHub repository
4. Configure:
   - **Name**: `tweet-sentiment-api`
   - **Runtime**: Select `Rust`
   - **Build Command**: `cargo build --release`
   - **Start Command**: `./target/release/sentiment-api`
   - **Plan**: Free (or Starter for better performance)

### 2.3 Add Environment Variables

Click "Environment" and add:

- **Key**: `RAPIDAPI_KEY`
- **Value**: Your RapidAPI key (from https://rapidapi.com)

### 2.4 Deploy

- Click "Create Web Service"
- Wait 5-10 minutes for build
- Copy your URL (e.g., `https://tweet-sentiment-api.onrender.com`)

### 2.5 Verify Backend

```bash
curl https://tweet-sentiment-api.onrender.com/health
```

Should return:

```json
{ "status": "healthy", "timestamp": "..." }
```

## Step 3: Deploy Frontend on Netlify (FREE)

### 3.1 Create Netlify Account

- Go to https://netlify.com
- Sign up with GitHub
- Authorize Netlify

### 3.2 Deploy Site

1. Click "Add new site"
2. Select "Import an existing project"
3. Choose your GitHub repository
4. Configure:
   - **Build command**: Leave empty
   - **Publish directory**: `.` (root)
5. Click "Deploy site"
6. Wait 1-2 minutes
7. Copy your URL (e.g., `https://tweet-sentiment.netlify.app`)

### 3.3 Verify Frontend

Open your Netlify URL in browser. You should see the app.

## Step 4: Connect Frontend to Backend

### 4.1 Update API URL

Edit `index.html` and find this line (around line 430):

```javascript
const API_URL = "http://127.0.0.1:8080";
```

Replace with your Render URL:

```javascript
const API_URL = "https://tweet-sentiment-api.onrender.com";
```

### 4.2 Commit & Push

```bash
git add index.html
git commit -m "Update API URL for production"
git push origin main
```

Netlify will auto-redeploy automatically.

## Step 5: Test Live Application

1. Open your Netlify URL
2. Enter topics: `bitcoin, crypto, tesla`
3. Click "Start Stream"
4. Wait 10-15 seconds
5. You should see tweets with sentiment analysis

## Step 6: Share Your Project

### Share Links

- **Frontend**: `https://your-netlify-url.netlify.app`
- **GitHub**: `https://github.com/yourusername/tweet-sentiment-analysis`
- **Backend API**: `https://tweet-sentiment-api.onrender.com`

### Add to GitHub Profile

1. Go to your GitHub profile
2. Add project to README
3. Pin repository

## Optional: Enable Auto-Deployment

### GitHub Actions (CI/CD)

The `.github/workflows/deploy.yml` file is already created. To enable:

1. Go to GitHub repository settings
2. Click "Secrets and variables" → "Actions"
3. Add these secrets:
   - `RENDER_SERVICE_ID`: From Render dashboard
   - `RENDER_API_KEY`: From Render account settings
   - `NETLIFY_AUTH_TOKEN`: From Netlify account settings
   - `NETLIFY_SITE_ID`: From Netlify site settings

Now every push to main will auto-deploy!

## Troubleshooting

### Backend not starting on Render

- Check logs in Render dashboard
- Verify `RAPIDAPI_KEY` is set
- Ensure Cargo.toml is correct

### Frontend shows blank page

- Check browser console for errors
- Verify API URL is correct
- Clear browser cache

### No tweets appearing

- Wait 10+ seconds after starting stream
- Check API key is valid
- Try different topics
- Check Render logs

### CORS errors

Add to `src/main.rs` if needed:

```rust
.wrap(middleware::DefaultHeaders::new().add(("Access-Control-Allow-Origin", "*")))
```

## Performance Tips

### For Render (Backend)

- Upgrade to Starter plan for better performance
- Enable auto-scaling
- Monitor logs regularly

### For Netlify (Frontend)

- Enable caching
- Minify assets
- Use CDN (automatic)

## Security Checklist

- [ ] Never commit `.env` file
- [ ] Use environment variables for secrets
- [ ] Enable HTTPS (automatic)
- [ ] Add rate limiting
- [ ] Validate all inputs
- [ ] Keep dependencies updated

## Next Steps

1. **Monitor**: Check logs regularly
2. **Improve**: Add features based on feedback
3. **Scale**: Upgrade plans if needed
4. **Share**: Tell others about your project
5. **Maintain**: Keep dependencies updated

## Support Resources

- **Render Docs**: https://render.com/docs
- **Netlify Docs**: https://docs.netlify.com
- **GitHub Docs**: https://docs.github.com
- **Rust Book**: https://doc.rust-lang.org/book/

## Estimated Costs

- **Render**: Free tier (with limitations) or $7/month
- **Netlify**: Free tier (unlimited)
- **GitHub**: Free tier (unlimited)
- **RapidAPI**: Free tier (limited requests) or $0-150/month

**Total**: Can run completely free!

---

**Congratulations!** Your project is now live on the internet! 🎉

Share your links and enjoy!
